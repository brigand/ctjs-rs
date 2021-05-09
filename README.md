## eval

```rust
use cjs::eval;

const X: f64 = eval! {
  const x = 5;
  String(x * Math.PI)
};
```
## Custom Derive

```rust
use cjs::Reflect;

#[derive(Debug, Reflect)]
#[reflect("reflect_s")]
struct S {
  foo: String,
  bar: String
}

reflect_s!{
  let out = ["impl ToString for S {"];
  out.push("fn to_string(&self) -> String {");
  out.push("vec![");
  out.push(ctx.fields.map(field => "(&" + field.name + ").to_string()").join(", "));
  out.push("].join(" + '"' + " & " + '"' + ")");
  out.push("}");
  out.push("}");
  out.join("\n");
}
```

## Prior Work

- https://github.com/fusion-engineering/inline-python
    - https://docs.rs/ct-python/0.5.1/ct_python/
- https://docs.rs/embed_js/0.1.4/embed_js/