## eval

```rust
use ctjs::eval;

const X: f64 = eval! {
  const x = 5;
  String(x * Math.PI)
};
```

## Custom Derive

```rust
use ctjs::JsMacro;

#[derive(Debug, JsMacro)]
#[js_macro = "fruit_derive"]
enum Fruit {
    #[js_macro(name = "granny smith")]
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

fn main() {
    for fruit in vec![Fruit::Apple, Fruit::Orange, Fruit::Pear] {
        println!("Debug: {:?}, Display: {}", fruit, fruit);
    }
}
```

## Prior Work

- https://github.com/fusion-engineering/inline-python
    - https://docs.rs/ct-python/0.5.1/ct_python/
- https://docs.rs/embed_js/0.1.4/embed_js/