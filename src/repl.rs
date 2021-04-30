//! A crate to help you easily build a repl
use std::io::{self, Write};
use std::{any::Any, collections::HashMap, sync::Arc};
type Data = HashMap<String, Arc<dyn Any>>;
type Functions = HashMap<String, fn(Data, Vec<String>)>;
use crate::Interpreter;
/// The main Repl Struct which contains pretty much everything the crate has to offer
#[derive(Debug)]
pub struct Repl {
    pub data: Data,
    /// the prompt that is displayed when asking for input
    prompt: String,
    /// arguments recieved from the repl, cleaned for \r\n and \n endings
    arguments: Vec<String>,
    /// the command for exiting, exists because exit functions won't work because of loop :(
    exit: String,
    /// all the functions in HashMap<String, fn(Vec<String>)> format, specified Vec<String>
    /// because of the limitations of function pointers
    pub(crate) functions: Functions,
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
    /// ```
    fn take_arg(&mut self) {
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
    ///     let mut repl = Repl::custom(
    ///         prompt: &str,
    ///         exit: &str,
    ///         functions: HashMap<String, fn(Vec<String>)>
    ///     );
    /// }
    /// ```
    pub fn custom(
        data: Data,
        prompt: &str,
        exit: &str,
        functions: HashMap<String, fn(Data, Vec<String>)>,
    ) -> Repl {
        Repl {
            data,
            arguments: vec![String::new()],
            prompt: prompt.to_string(),
            exit: exit.to_string(),
            functions,
        }
    }
    /// returns new repl
    /// # Example
    /// ```rust,ignore
    /// use repl_framework::Repl;
    /// fn test(_: Vec<String>) {
    ///     println!("test");
    /// }
    /// fn main() {
    ///     let mut repl = Repl::new(">>> ");
    ///     repl.add_function("test", test as fn(Vec<String>));
    ///     repl.run();
    /// }
    /// ```
    pub fn new(prompt: &str) -> Repl {
        Repl {
            data: Default::default(),
            arguments: vec![String::new()],
            prompt: prompt.to_string(),
            exit: "exit".to_string(),
            functions: Default::default(),
        }
    }
    pub fn from_interpreter(interpreter: Interpreter, prompt: &str, exit: &str) -> Repl {
        Repl {
            data: interpreter.data,
            prompt: prompt.to_string(),
            arguments: Vec::new(),
            exit: exit.to_string(),
            functions: interpreter.functions,
        }
    }
    pub fn add_function(&mut self, name: String, func: fn(Data, Vec<String>)) {
        self.functions.insert(name, func);
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
    /// ```
    pub fn run(&mut self) {
        loop {
            self.take_arg();
            if self.arguments.concat() == self.exit {
                println!("Terminated REPL");
                break;
            }
            if self.functions.contains_key(&self.arguments[0]) {
                self.functions[&self.arguments[0]](
                    self.data.clone(),
                    self.arguments[1..self.arguments.len()].to_vec(),
                );
            } else if self.functions.contains_key("") {
                self.functions[""](
                    self.data.clone(),
                    self.arguments[0..self.arguments.len()].to_vec(),
                );
            }
        }
    }
    /// Runs the repl in debug mode
    pub fn run_debug(&mut self) {
        loop {
            self.take_arg();
            if self.arguments.concat() == self.exit {
                println!("Terminated REPL");
                break;
            }
            if self.functions.contains_key(&self.arguments[0]) {
                self.functions[&self.arguments[0]](
                    self.data.clone(),
                    self.arguments[1..self.arguments.len()].to_vec(),
                );
            }
            println!("{:?}", &self);
        }
    }
}
