#![allow(dead_code)]
use std::io::{self, Write};
use std::collections::HashMap;
/// The main Repl Struct which contains pretty much everything the crate has to offer
#[derive(Debug)]
pub struct Repl {
    /// the prompt that is displayed when asking for input
    prompt: String,
    /// arguments recieved from the repl, cleaned for \r\n and \n endings
    pub arguments: Vec<String>,
    /// the command for exiting, exists because exit functions won't work because of loop :(
    exit: String,
    /// all the functions in HashMap<String, fn(Vec<String>)> format, specified Vec<String>
    /// because of the limitations of function pointers
    functions: HashMap<String, fn(Vec<String>)>
}
/// Repl methods
impl Repl {
    /// Takes argument from stdin and mutate self.arguments to it
    /// # Example
    /// ```rust,ignore
    /// use std::collections::HashMap;
    /// use repl_framework::Repl;
    /// fn test(Vec<String>) {
    /// println!("test!");
    /// }
    /// fn main() {
    ///     let mut hashmap = HashMap::new();
    ///     hashmap.insert("test".to_string(), test as fn(Vec<()>));
    ///     let mut repl = Repl::new(">>> ", hashmap);
    ///     repl.take_arg();
    /// }
    pub fn take_arg(&mut self) {
        self.arguments = self.take_arg_return();
    }
    /// same as take_arg, but returns the argument instead of storing it in self.argument
    pub fn take_arg_return(&self) -> Vec<String> {
        print!("{}", &self.prompt);
        // flushing stdout because print! does'nt do it by default
        io::stdout()
            .flush()
            .expect("something went wrong went flushing the buffer");
        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        };
        // trimming \r\n and \n from the input
        buf = buf
            .trim_end_matches('\n')
            .trim_end_matches('\r')
            .to_string();
        let mut args = vec![];
        for i in buf.split(' ') {
            args.push(i.to_string());
        }
        args
    }
    /// returns a customized Repl
    /// currently not much different from normal new other than the choice of exit keyword
    /// # Example
    /// ```rust,ignore
    /// use repl_framework::Repl;
    /// fn main() {
    ///     let mut repl = Repl::customized_new(
    ///         prompt: &str,
    ///         exit: &str,
    ///         functions: HashMap<String, fn(Vec<String>)>
    ///     );
    ///}
    pub fn customized_new(prompt: &str, exit: &str, functions: HashMap<String, fn(Vec<String>)>) -> Repl {
        Repl {
            arguments: vec![String::new()],
            prompt: prompt.to_string(),
            exit: exit.to_string(),
            functions: functions,
        }
    }
    /// returns new repl
    /// # Example
    /// ```rust,ignore
    /// use std::collections::HashMap;
    /// use repl_framework::Repl;
    /// fn main() {
    ///     let mut functions = HashMap::new().insert("test".to_string, test as fn(Vec<String>));
    ///     let mut repl = Repl::new("Hello", functions);
    /// }
    /// fn test(args: Vec<String>) {
    ///     println!("{:?}", args);
    /// }
    pub fn new(prompt: &str, functions: HashMap<String, fn(Vec<String>)>) -> Repl {
        Repl {
            arguments: vec![String::new()],
            prompt: prompt.to_string(),
            exit: "exit".to_string(),
            functions: functions,
        }
    }
    /// runs the repl
    /// # Example
    /// ```rust,ignore
    /// use repl_framework::Repl;
    /// use std::collections::HashMap;
    /// fn test(_: Vec<String>) {
    /// println!("test!");
    /// }
    /// fn main() {
    ///     let mut hashmap = HashMap::new();
    ///     hashmap.insert("test".to_string(), test as fn(Vec<String>));
    ///     let mut repl = Repl::new(">>> ", hashmap);
    ///     repl.run()
    /// }
    pub fn run(&mut self) {
        loop {
            &self.take_arg();
            if self.arguments.concat() == self.exit {
                println!("Terminated REPL");
                break;
            }
            if self.functions.contains_key(&self.arguments[0]) {
                self.functions[&self.arguments[0]](self.arguments[1..self.arguments.len()].to_vec());
            }
        }
    }
    /// Runs the repl in debug mode
    pub fn run_debug(&mut self) {
        loop {
            &self.take_arg();
            if self.arguments.concat() == self.exit {
                println!("Terminated REPL");
                break;
            }
            if self.functions.contains_key(&self.arguments[0]) {
                self.functions[&self.arguments[0]](self.arguments[1..self.arguments.len()].to_vec());
            }
            println!("{:?}", &self);
        }
    }
}
