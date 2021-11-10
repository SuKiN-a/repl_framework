use repl_framework::Repl;

fn main() -> std::io::Result<()> {
    Repl::default()
        // makes command with name "get" which calls Store::get when called
        .with_function("get", Store::get)
        .with_function("set", Store::set)
        // commands with the name "" get called whenever the user inputs a commmand which is not declared
        .with_function("", Store::unknown_command)
        .run()
}

#[derive(Debug, Default)]
struct Store {
    data: std::collections::HashMap<String, String>,
}

impl Store {
    fn get(&mut self, args: Vec<String>) {
        args.into_iter()
            .for_each(|f| println!("{}", &self.data[&f]));
    }
    fn set(&mut self, args: Vec<String>) {
        args.chunks_exact(2).for_each(|f| {
            self.data.insert(f[0].clone(), f[1].clone());
        })
    }
    fn unknown_command(&mut self, args: Vec<String>) {
        eprintln!("unknown command {:?}", args.first())
    }
}
