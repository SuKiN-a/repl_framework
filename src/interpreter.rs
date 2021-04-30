use crate::Repl;
use std::{any::Any, collections::HashMap, fs, sync::Arc};
type Data = HashMap<String, Arc<dyn Any>>;
type Functions = HashMap<String, fn(Data, Vec<String>)>;
/// the interpreter struct that contains all logic and code for interpreter creation
pub struct Interpreter {
    pub data: Data,
    pub(crate) functions: Functions,
}

impl Interpreter {
    /// create a new blank interpreter
    pub fn new() -> Interpreter {
        Interpreter {
            data: Default::default(),
            functions: HashMap::new(),
        }
    }
    /// create an interpreter from a repl
    pub fn from_repl(repl: Repl) -> Interpreter {
        Interpreter {
            data: repl.data,
            functions: repl.functions,
        }
    }
    /// add a function to an interpreter
    pub fn add_function(&mut self, name: String, function: fn(Data, Vec<String>)) {
        self.functions.insert(name, function);
    }
    /// run the interpreter
    pub fn run(&self, filename: &str) {
        let mut arg: Vec<String>;
        for line in fs::read_to_string(filename)
            .expect("could not read file")
            .split("\n")
        {
            arg = line
                .trim_end_matches("\r")
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            if self.functions.contains_key(&arg[0]) {
                self.functions[&arg[0]](self.data.clone(),arg[1..arg.len()].to_vec());
            }
        }
    }
    // runs the interpreter in debug mode
    pub fn run_debug(&self, filename: &str) {
        let mut arg: Vec<String>;
        for line in fs::read_to_string(filename)
            .expect("could not read file")
            .split("\n")
        {
            arg = line
                .trim_end_matches("\r")
                .split(" ")
                .map(|x| x.to_string())
                .collect();
            if self.functions.contains_key(&arg[0]) {
                dbg!(&self.functions, &arg, &arg.len());
                arg.iter().for_each(|x| {
                    println!("{}", dbg!(x).len());
                });
                dbg!("{}", arg[1..arg.len()].to_vec());
                self.functions[&arg[0]](self.data.clone(),arg[1..arg.len()].to_vec());
            }
        }
    }
}
