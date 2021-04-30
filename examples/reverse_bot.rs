use std::{any::Any, collections::HashMap, sync::Arc};

use repl_framework::{add_function, Repl};
fn main() {
    // setting up the repl
    let mut repl = Repl::new(">>> ");
    // adding function reverse with no alias
    add_function!(repl, "", reverse);
    repl.run();
    // run the following code if you want an interpreter
    // Interpreter::from_repl(repl).run("filename");
}

// function to reverse string
// probably not optimal, but it works
fn reverse(_: HashMap<String, Arc<dyn Any>>, string: Vec<String>) {
    let mut outstring = String::new();
    for i in string {
        let j = i.chars();
        let mut j = j.into_iter().collect::<Vec<char>>();
        j.reverse();
        for i in j {
            outstring += &i.to_string()[..];
        }
        outstring += " ";
    }
    println!("{}", outstring);
}
