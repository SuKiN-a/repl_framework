use repl_framework::Repl;
fn main() -> std::io::Result<()> {
    Repl::default().with_function("read", cat).run()
}

fn cat(_: &mut (), args: Vec<String>) {
    args.into_iter().for_each(|f| {
        println!("{}", std::fs::read_to_string(f).unwrap());
    });
}
