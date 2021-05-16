use proc_macro2::{Delimiter, Group, Literal, Spacing, TokenStream, TokenTree};
use std::{fmt::Write, iter::Peekable};
use syn::LitStr;

pub fn parse_to_js(input: TokenStream) -> String {
    let mut code = String::new();
    add(&mut code, input, Hint::None);
    code
}

fn parse_raw_literal(lit: &Literal) -> Option<String> {
    let tt: TokenTree = lit.clone().into();
    let ts: TokenStream = tt.into();
    let lit: LitStr = syn::parse2(ts).expect("parse_raw_literal");
    let lit = lit.value();

    return Some(lit);

    // let mut chars = lit.chars();

    // let (prefix_len, suffix_len) = match chars.next()? {
    //     '"' => ('"'.len_utf8(), '"'.len_utf8()),
    //     'r' => {
    //         let mut hashes = 0;

    //         while let Some(ch) = chars.next() {
    //             if ch == '#' {
    //                 hashes += 1;
    //             } else if ch == '"' {
    //                 break;
    //             } else {
    //                 unreachable!("Unexpected character in raw literal before quote: {:?}", ch)
    //             }
    //         }

    //         (
    //             'r'.len_utf8() + hashes + '"'.len_utf8(),
    //             hashes + '"'.len_utf8(),
    //         )
    //     }
    //     _ => return None,
    // };

    // let contents = &lit[prefix_len..lit.len() - suffix_len];
    // Some(contents.trim().to_owned())
}

fn match_macro(
    token: TokenTree,
    tokens: &mut Peekable<impl Iterator<Item = TokenTree>>,
    name: &str,
) -> Option<Group> {
    match token {
        TokenTree::Ident(ident) if ident.to_string().as_str() == name => {
            match tokens.peek() {
                Some(TokenTree::Punct(punct)) if punct.as_char() == '!' => {
                    let _ = tokens.next();
                }
                _ => return None,
            }

            if let Some(TokenTree::Group(group)) = tokens.next() {
                // Discard closing punctuation
                let _ = tokens.next();
                Some(group)
            } else {
                None
            }
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Hint {
    Str,
    Tokens,
    None,
}

// Based on https://github.com/fusion-engineering/inline-python/blob/7604f78b2c834f5f5ee77defecb5a1a8824fac9d/macros/src/embed_python.rs
fn add(code: &mut String, input: TokenStream, hint: Hint) {
    let mut tokens = input.into_iter().peekable();

    let mut first = Some(());

    while let Some(token) = tokens.next() {
        let is_first = first.take().is_some() && code.is_empty();

        if is_first && hint != Hint::Tokens {
            if let Some(group) = match_macro(token.clone(), &mut tokens, "concat") {
                fn handle_token(code: &mut String, token: TokenTree) {
                    match token {
                        TokenTree::Group(group) => {
                            let mut tokens = group.stream().into_iter().peekable();
                            while let Some(token) = tokens.next() {
                                if let Some(js) = match_macro(token.clone(), &mut tokens, "js") {
                                    write!(code, "{}", "{\n").unwrap();
                                    add(code, js.stream(), Hint::Tokens);
                                    write!(code, "{}", "\n}").unwrap();
                                } else {
                                    handle_token(code, token);
                                }
                            }
                        }
                        TokenTree::Literal(lit) => {
                            if let Some(js) = parse_raw_literal(&lit) {
                                write!(code, "{}\n", js).unwrap();
                            }
                        }
                        TokenTree::Punct(_) => {}
                        _ => {
                            panic!(
                                "Unexpected token in ctjs::eval!(concat!(HERE)). Token: {:?}",
                                token
                            );
                        }
                    }
                }

                // panic!("Group: {:#?}", group.stream());
                handle_token(code, TokenTree::Group(group));
                continue;
            } else if let Some(group) = match_macro(token.clone(), &mut tokens, "js") {
                add(code, group.stream(), Hint::Tokens);
                continue;
            } else if let TokenTree::Literal(lit) = &token {
                if let Some(js) = parse_raw_literal(lit) {
                    write!(code, "{}", js).unwrap();
                    continue;
                }
            }
        }

        match &token {
            TokenTree::Group(inner) => {
                let (start, end) = match inner.delimiter() {
                    Delimiter::Parenthesis => ("(", ")"),
                    Delimiter::Brace => ("{", "}"),
                    Delimiter::Bracket => ("[", "]"),
                    Delimiter::None => ("", ""),
                };
                code.write_str(start).unwrap();
                add(code, inner.stream(), Hint::Tokens);
                code.write_str(end).unwrap();
            }
            TokenTree::Punct(inner) => {
                if inner.spacing() == Spacing::Alone
                    && !code.ends_with(" ")
                    && inner.as_char() != '('
                    && inner.as_char() != '>'
                    && inner.as_char() != '.'
                {
                    code.push_str(" ");
                }

                if code.ends_with("\n") && inner.as_char() == '=' {
                    code.pop();
                }

                if code.ends_with("= ") && matches!(inner.as_char(), '>' | '=') {
                    code.pop();
                }

                if inner.as_char() == '#' && inner.spacing() == Spacing::Joint {
                    // Convert '##' to '//', because otherwise it's
                    // impossible to use the Python operators '//' and '//='.
                    match tokens.next() {
                        Some(TokenTree::Punct(ref p)) if p.as_char() == '#' => {
                            code.push_str("//");
                        }
                        Some(TokenTree::Punct(p)) => {
                            code.push(inner.as_char());
                            code.push(p.as_char());
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                } else {
                    code.push(inner.as_char());
                }

                if inner.spacing() == Spacing::Alone && !code.ends_with(" ") {
                    if inner.as_char() == '.' {
                        // Do nothing
                    } else if inner.as_char() == ')' {
                        code.push_str("\n");
                    } else {
                        code.push_str(" ");
                    }
                }
            }
            TokenTree::Ident(inner) => {
                let suffix = match tokens.peek().and_then(as_punct) {
                    Some('.') | Some('(') | Some('[') | Some('{') => "",
                    _ => " ",
                };
                write!(code, "{}{}", inner, suffix).unwrap();
            }
            TokenTree::Literal(inner) => {
                write!(code, "{}", inner).unwrap();
            }
        }
    }
}

fn as_punct(tt: &TokenTree) -> Option<char> {
    match tt {
        TokenTree::Punct(p) => Some(p.as_char()),
        _ => None,
    }
}
