use repl_framework::Repl;
fn main() -> std::io::Result<()> {
    Repl::default().with_function("", reverse).run()
}
#[derive(Default)]
struct Data;

// function to reverse string
// probably not optimal, but it works
fn reverse(_: &mut Data, strings: Vec<String>) {
    let mut outstring = String::new();
    strings.into_iter().for_each(|string| {
        string.chars().rev().for_each(|f| outstring.push(f));
        outstring += " ";
    });
    println!("{}", outstring);
}
