use proc_macro2::{Delimiter, Spacing, TokenStream, TokenTree};
use std::fmt::Write;

pub fn parse_to_js(input: TokenStream) -> String {
    let mut code = String::new();
    add(&mut code, input);
    code
}

// Based on https://github.com/fusion-engineering/inline-python/blob/7604f78b2c834f5f5ee77defecb5a1a8824fac9d/macros/src/embed_python.rs
fn add(code: &mut String, input: TokenStream) {
    let mut tokens = input.into_iter();

    while let Some(token) = tokens.next() {
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
                {
                    code.push_str(" ");
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
                    code.push_str(" ");
                }
            }
            TokenTree::Ident(inner) => {
                write!(code, "{} ", inner).unwrap();
            }
            TokenTree::Literal(inner) => {
                write!(code, "{}", inner).unwrap();
            }
        }
    }
}
