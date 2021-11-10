use repl_framework::Repl;
fn main() -> std::io::Result<()> {
    Repl::default()
        .with_prompt("read:> ")
        .with_function("", cat)
        .with_function("help", help)
        .run()
}

fn cat(_: &mut (), args: Vec<String>) {
    args.into_iter().for_each(|f| {
        println!("{}", std::fs::read_to_string(f).unwrap());
    });
}

fn help(_: &mut (), _: Vec<String>) {
    println!("help:\n\t[files] : prints contents of files")
}
