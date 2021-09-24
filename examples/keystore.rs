use repl_framework::Repl;

fn main() -> std::io::Result<()> {
    Repl::default()
        .with_function("get", Store::get)
        .with_function("set", Store::set)
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
}
