use proc_macro2::{Delimiter, Literal, Spacing, TokenStream, TokenTree};
use std::fmt::Write;
use syn::{token::Token, LitStr};

pub fn parse_to_js(input: TokenStream) -> String {
    let mut code = String::new();
    add(&mut code, input);
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

// Based on https://github.com/fusion-engineering/inline-python/blob/7604f78b2c834f5f5ee77defecb5a1a8824fac9d/macros/src/embed_python.rs
fn add(code: &mut String, input: TokenStream) {
    let mut tokens = input.into_iter();

    let mut first = Some(());

    while let Some(token) = tokens.next() {
        let is_first = first.take().is_some() && code.is_empty();

        if is_first {
            if let TokenTree::Ident(ident) = &token {
                if ident.to_string().as_str() == "concat" {
                    let punct = matches!(tokens.next(), Some(TokenTree::Punct(_)));

                    fn handle_token(code: &mut String, token: TokenTree) {
                        match token {
                            TokenTree::Group(group) => {
                                for token in group.stream() {
                                    handle_token(code, token);
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

                    if let Some(token) = tokens.next() {
                        // panic!("Group: {:#?}", group.stream());
                        handle_token(code, token);
                    }
                    let _ = tokens.next();
                    return;
                }
            }
            if let TokenTree::Literal(lit) = &token {
                if let Some(js) = parse_raw_literal(lit) {
                    write!(code, "{}", js).unwrap();
                    return;
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
                add(code, inner.stream());
                code.write_str(end).unwrap();
            }
            TokenTree::Punct(inner) => {
                if inner.spacing() == Spacing::Alone
                    && !code.ends_with(" ")
                    && inner.as_char() != '('
                    && inner.as_char() != '>'
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
                    if inner.as_char() == ')' {
                        code.push_str("\n");
                    } else {
                        code.push_str(" ");
                    }
                }
            }
            TokenTree::Ident(inner) => {
                write!(code, "{}", inner).unwrap();
                if code.ends_with("return") {
                    code.push(' ');
                } else {
                    code.push('\n');
                }
            }
            TokenTree::Literal(inner) => {
                write!(code, "{}", inner).unwrap();
            }
        }
    }
}
