//! Execute JavaScript at compile time to generate Rust code. Both evaluating expressions and
//! custom derives are supported.
//!
//! ## eval
//!
//! ```rust
//! use ctjs::eval;
//!
//! const X: f64 = eval! {
//!   // This is JavaScript
//!   const x = 5;
//!   String(x * Math.PI)
//! };
//!
//! assert!(X > 15.0);
//! assert!(X < 16.0);
//! ```
//!
//! ## Custom Derive
//!
//! ```rust
//! use ctjs::JsMacro;
//!
//! #[derive(Debug, JsMacro)]
//! #[js_macro = "fruit_derive"]
//! enum Fruit {
//!     #[js(name = "granny smith")]
//!     Apple,
//!     Orange,
//!     Pear,
//! }
//!
//! fruit_derive! {
//!     js!(
//!         let output = "const _: () = {\n";
//!         output += "use std::fmt::{self, Write};\n";
//!         // name is "Fruit"
//!         output += "impl fmt::Display for " + name + "{\n";
//!         output += "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n";
//!         output += "write!(f, \"{}\", match self {\n";
//!         // ident is "Apple" or "Orange" or "Pear"
//!         for (const { ident, attrs } of item.variants) {
//!             let string = '"' + ident.toLowerCase() + '"';
//!             const kv = ctjs.parse_attrs(attrs);
//!             if (kv.name) {
//!                 string = kv.name;
//!             }
//!
//!             output += "Self::" + ident + " => " + string + ",\n";
//!         }
//!         output += "})\n";
//!         output += "}\n}\n};\n";
//!         output
//!     )
//! }
//!
//! let fruits = vec![Fruit::Apple, Fruit::Orange, Fruit::Pear];
//! for fruit in &fruits {
//!     println!("Debug: {:?}, Display: {}", fruit, fruit);
//! }
//!
//! assert_eq!(&fruits[0].to_string(), "granny smith");
//! assert_eq!(&fruits[1].to_string(), "orange");
//! assert_eq!(&fruits[2].to_string(), "pear");
//! ```
pub use ctjs_macros::*;

#[cfg(test)]
mod tests {
    use ctjs_macros::eval;
    use ctjs_macros::JsMacro;

    #[test]
    fn it_works() {
        let y = eval! {
            let x = 3;
            x * 3.5
        };
        assert_eq!(y, 10.5);
    }

    #[test]
    fn it_can_generate_sin_table() {
        let nums: Vec<f64> = eval! {
            const values = Array.from({ length: 30 }, (x, i) => Math.sin(i / (Math.PI * 2)));
            "vec![" + values.map(value => value % 1 === 0 ? value + ".0" : value) + "]"
        };

        assert_eq!(nums.len(), 30);
    }

    #[test]
    fn it_can_generate_sin_table_with_helpers() {
        let nums: Vec<f64> = eval! {
            const values = ctjs.range(0, 30).map(x => Math.sin(x / (Math.PI * 2)));
            ctjs.vec(values.map(ctjs.float))
        };

        assert_eq!(nums.len(), 30);

        // println!("Nums: {:#?}", nums);
        // assert_eq!("making test fail to see stdout", "");
    }

    #[test]
    fn it_can_generate_sin_table_with_raw_string() {
        let nums: Vec<f64> = eval! {r#"
            const values = Array.from({ length: 30 }, (x, i) => Math.sin(i / (Math.PI * 2)));
            `vec![${values.map(value => value % 1 === 0 ? value + ".0" : value)}]`
        "#};

        assert_eq!(nums.len(), 30);
        // println!("Nums: {:#?}", nums);
        // assert_eq!("making test fail to see stdout", "");
    }

    #[test]
    fn it_handles_multiple_strings() {
        let foo: u8 = eval! {
            concat!(
            r#" let x = 42; "#,
            r#" x + '_u8'  "#
            )
        };
        assert_eq!(foo, 42);
    }

    #[test]
    fn it_can_derive_simple() {
        #[derive(Debug, JsMacro)]
        #[js_macro = "simple"]
        struct TestStruct {
            pub s: String,
        }

        simple! { js!("static NAME: &str = " + ctjs.str(name) + ";") }

        assert_eq!(NAME, "TestStruct");
    }
}

// fn example_test() {
//     #[derive(Debug, JsMacro)]
//     #[js_macro = "example"]
//     enum TestStruct {
//         Foo(String),
//         #[js_macro(example = "val")]
//         Bar {
//             something: &'static str,
//         },
//     }

//     example! { js!("static NAME: &str = " + ctjs.str(name) + ";") }
//     example! { js!("static JSON: &str = " + ctjs.str(ctjs.json(item)) + ";" ) }

//     // assert_eq!(NAME, "TestStruct");
// }
