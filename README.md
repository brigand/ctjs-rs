<!-- {\{badges}} -->
<!--
  README generated with https://github.com/livioribeiro/cargo-readme
  $ cargo readme > README.md
-->

# ctjs

Execute JavaScript at compile time to generate Rust code. Both evaluating expressions and
custom derives are supported.

### eval

```rust
use ctjs::eval;

const X: f64 = eval! {
  // This is JavaScript
  const x = 5;
  String(x * Math.PI)
};

assert!(X > 15.0);
assert!(X < 16.0);
```

### Custom Derive

```rust
use ctjs::JsMacro;

#[derive(Debug, JsMacro)]
#[js_macro = "fruit_derive"]
enum Fruit {
    #[js(name = "granny smith")]
    Apple,
    Orange,
    Pear,
}

fruit_derive! {
    js!(
        let output = "const _: () = {\n";
        output += "use std::fmt::{self, Write};\n";
        // name is "Fruit"
        output += "impl fmt::Display for " + name + "{\n";
        output += "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n";
        output += "write!(f, \"{}\", match self {\n";
        // ident is "Apple" or "Orange" or "Pear"
        for (const { ident, attrs } of item.variants) {
            let string = '"' + ident.toLowerCase() + '"';
            const kv = ctjs.parse_attrs(attrs);
            if (kv.name) {
                string = kv.name;
            }

            output += "Self::" + ident + " => " + string + ",\n";
        }
        output += "})\n";
        output += "}\n}\n};\n";
        output
    )
}

let fruits = vec![Fruit::Apple, Fruit::Orange, Fruit::Pear];
for fruit in &fruits {
    println!("Debug: {:?}, Display: {}", fruit, fruit);
}

assert_eq!(&fruits[0].to_string(), "granny smith");
assert_eq!(&fruits[1].to_string(), "orange");
assert_eq!(&fruits[2].to_string(), "pear");
```

Current version: 0.0.1

## Prior Work

- https://github.com/fusion-engineering/inline-python
    - https://docs.rs/ct-python/0.5.1/ct_python/
- https://docs.rs/embed_js/0.1.4/embed_js/

All code licensed as MIT
