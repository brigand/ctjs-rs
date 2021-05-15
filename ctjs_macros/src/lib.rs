extern crate proc_macro;

mod parser;

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

fn derive_js_macro_impl(input: DeriveInput) -> Result<TokenStream, TokenStream> {
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

    let visibility = match input.vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Crate(_) => "crate",
        syn::Visibility::Restricted(_) => "restricted",
        syn::Visibility::Inherited => "inherited",
    };

    let ident = input.ident.to_string();

    // TODO: convert visibility, ident, and input.data to json and generate a JS program
    // that stores it in a global variable. Then have a macro concat that with the JS code
    // passed to #macro_name
    // let (data_kind, data_value) = match input.data {};

    let code = format!("let struct_name = \"{}\";", ident);
    // panic!("code: {}", code);
    let code_token = syn::Lit::Str(syn::LitStr::new(&code, Span::mixed_site()));

    Ok(quote! {
        macro_rules! #macro_name_token {
            ($js:literal) => {
                ctjs_macros::eval!(
                    concat!(
                        #code_token,
                        $js
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
