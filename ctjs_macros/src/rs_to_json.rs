use proc_macro2::Span;
use serde_json::json;
use serde_json::to_value;
use serde_json::Value;
use syn::DeriveInput;
use syn_serde::Syn;

pub fn jsonify(input: DeriveInput) -> Value {
    let visibility = match input.vis {
        syn::Visibility::Public(_) => "pub",
        syn::Visibility::Crate(_) => "crate",
        syn::Visibility::Restricted(_) => "restricted",
        syn::Visibility::Inherited => "inherited",
    };

    let generics =
        to_value(input.generics.to_adapter()).expect("derive generics to serde_json::Value");

    let item = match input.data {
        syn::Data::Struct(data) => json!({
            "type": "struct",
            "fields": to_value(data.fields.to_adapter()).unwrap()
        }),
        syn::Data::Enum(data) => json!({
            "type": "enum",
            "variants": punct_value(data.variants.clone())
        }),
        syn::Data::Union(data) => json!({
            "type": "union",
            "fields": to_value(data.fields.to_adapter()).unwrap()
        }),
    };

    json!({
        "name": input.ident.to_string(),
        "visibility": visibility,
        "generics": generics,
        "item": item
    })
}

fn punct_value<T, P>(p: syn::punctuated::Punctuated<T, P>) -> Value
where
    T: Syn,
{
    p.into_iter()
        .map(|t| to_value(t.to_adapter()).unwrap())
        .collect()
}
