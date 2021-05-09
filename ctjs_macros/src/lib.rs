extern crate proc_macro;

mod parser;

use proc_macro::{Span, TokenStream as TokenStream1};
use proc_macro2::{Literal, TokenStream};
use quick_js::{Context, JsValue};
use quote::quote;
use std::str::FromStr;

fn eval_impl(input: TokenStream) -> Result<TokenStream, TokenStream> {
    let context = Context::new().unwrap();

    let js = parser::parse_to_js(input.clone());

    // panic!("JS to execute: {}", js);

    let output = context.eval(&js).expect("eval_as string");

    match output {
        JsValue::Undefined | JsValue::Null => Ok(quote! { () }),
        JsValue::Bool(b) => Ok(quote! { #b }),
        JsValue::Int(i) => Ok(quote! { #i }),
        JsValue::Float(f) => Ok(quote! { #f }),
        JsValue::String(rust_code) => {
            TokenStream::from_str(&rust_code).map_err(|_| TokenStream::new())
        }
        JsValue::Array(_) => {
            panic!("ctjs evaluation resulted in an Array, which doesn't map cleanly to Rust types. Try returning a string of rust code, or a bool or number");
        }
        JsValue::Object(_) => {
            panic!("ctjs evaluation resulted in an Object, which doesn't map cleanly to Rust types. Try returning a string of rust code, or a bool or number");
        }
        JsValue::__NonExhaustive => {
            unreachable!()
        }
    }
}

// #[doc(hidden)]
#[proc_macro]
pub fn eval(input: TokenStream1) -> TokenStream1 {
    TokenStream1::from(match eval_impl(TokenStream::from(input)) {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
