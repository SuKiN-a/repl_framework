# repl_framework

An easy to use repl creation library

## quickstart

```rust
use repl_framework::Repl;
fn main() -> std::io::Result<()>{
    Repl::default().with_function("Hello", hello).run()
}
fn hello(_: &mut (), _: Vec<String>) {
    println!("Hello World")
}
```
