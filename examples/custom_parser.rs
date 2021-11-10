use repl_framework::Repl;
fn main() -> std::io::Result<()> {
    Repl::default()
        .with_parser(|f| f.split('.').map(|f| f.to_owned()).collect())
        .with_prompt("print:> ")
        .with_function("", |_: &mut (), args| println!("{}", args.join("/")))
        .run()
}
