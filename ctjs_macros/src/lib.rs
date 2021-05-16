extern crate proc_macro;

mod parser;
mod rs_to_json;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quick_js::{Context, JsValue};
use quote::quote;
use serde_json::json;
use std::str::FromStr;
use syn::{parse_macro_input, DeriveInput};

static RUNTIME: &str = include_str!("../runtime.js");

fn eval_impl(input: TokenStream) -> Result<TokenStream, TokenStream> {
    let context = Context::new().unwrap();

    let js = parser::parse_to_js(input.clone());

    // panic!("JS to execute: {}", js);

    if let Err(err) = context.eval(RUNTIME) {
        panic!(
            "ctjs error evaluating runtime. Report this as a bug. Error: {:?}",
            err
        );
    }

    let output = match context.eval(&js) {
        Ok(value) => value,
        Err(err) => {
            panic!("Error when evaluating code: {}\nSource:\n{}", err, js);
        }
    };

    match output {
        JsValue::Undefined | JsValue::Null => Ok(quote! { () }),
        JsValue::Bool(b) => Ok(quote! { #b }),
        JsValue::Int(i) => Ok(quote! { #i }),
        JsValue::Float(f) => Ok(quote! { #f }),
        JsValue::String(rust_code) => {
            match TokenStream::from_str(&rust_code) {
                Ok(ts) => Ok(ts),
                Err(err) => panic!("Expected this string to be valid when interpreted as a TokenStream: {:?}\nParse error: {:?}", rust_code, err)
            }
        }
        JsValue::Array(_) => {
            panic!("ctjs evaluation resulted in an Array, which doesn't map cleanly to Rust types. Try returning a string of rust code, or a bool or number");
        }
        JsValue::Object(_) => {
            panic!("ctjs evaluation resulted in an Object, which doesn't map cleanly to Rust types. Try returning a string of rust code, or a bool or number");
        }
        _ => {
            unreachable!()
        }
    }
}

fn derive_js_macro_impl(input: DeriveInput) -> Result<TokenStream, TokenStream> {
    // TODO: convert visibility, ident, and input.data to json and generate a JS program
    // that stores it in a global variable. Then have a macro concat that with the JS code
    // passed to #macro_name
    // let (data_kind, data_value) = match input.data {};

    let macro_name = input.attrs.iter().find_map(|attr| {
        // panic!("Path: {:?}", attr.path);
        if !attr.path.is_ident("js_macro") {
            return None;
        }

        let meta = attr
            .parse_meta()
            .expect("js_macro meta must be valid (according to syn)");

        match meta {
            syn::Meta::NameValue(kv) if kv.path.is_ident("js_macro") => match kv.lit {
                syn::Lit::Str(lit_str) => Some(lit_str.value()),
                _ => None,
            },
            _ => None,
        }
    });

    let macro_name = macro_name.as_deref().unwrap_or("js_macro");
    let macro_name_token = syn::Ident::new(&macro_name, Span::mixed_site());

    let item_name = input.ident.to_string();

    let json_value = rs_to_json::jsonify(input.clone());

    let code = format!(
        r#"Object.assign(this, {});"#,
        serde_json::to_string(&json_value).unwrap()
    );
    let code_token = syn::Lit::Str(syn::LitStr::new(&code, Span::call_site()));

    // let ident = syn::Ident::new("NAME", Span::call_site());
    // return Ok(quote! {
    //     static #ident: &str = #code_token;
    // });
    Ok(quote! {
        macro_rules! #macro_name_token {
            ($( $js:tt )*) => {
                ctjs_macros::eval!(
                    concat!(
                        #code_token,
                        $($js)*
                    )
                );
            };
        }
    })
}

// #[doc(hidden)]
#[proc_macro]
pub fn eval(input: TokenStream1) -> TokenStream1 {
    TokenStream1::from(match eval_impl(TokenStream::from(input)) {
        Ok(tokens) => tokens,
        Err(tokens) => tokens,
    })
}

#[proc_macro_derive(JsMacro, attributes(js_macro))]
pub fn derive_js_macro(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as DeriveInput);

    TokenStream1::from(match derive_js_macro_impl(input) {
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
