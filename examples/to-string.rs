use ctjs::{eval, JsMacro};

#[derive(Debug, JsMacro)]
#[js_macro = "fruit_derive"]
enum Fruit {
    Apple,
    Orange,
    Pear,
}

fruit_derive! {
    js!(
        let output = "const _: () = {\n";
        output += "use std::fmt::{self, Write};\n";
        output += "impl fmt::Display for " + name + "{\n";
        output += "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {\n";
        output += "write!(f, \"{}\", match self {\n";
        for (const {ident} of item.variants) {
        output += "Self::" + ident + "=> \"" + ident.toLowerCase() + "\",\n";
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
