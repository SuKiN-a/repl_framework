use crate::utils::{self, FnPtr};
use std::{collections::HashMap, io, io::Write};
/// Main Repl Struct that contains all logic for the crate
#[derive(Debug, Clone)]
pub struct Repl<'a, T> {
    /// struct to store data, used as an argument in all functions
    pub data: T,
    /// prompt that is displayed before asking for input
    prompt: &'a str,
    /// command used to exit. defaults to "exit"
    exit: &'a str,
    /// text displayed when repl is closed: defaults to "repl terminated"
    exit_message: &'a str,
    /// hashmap that stores functions
    functions: HashMap<&'a str, FnPtr<T>>,
}
impl<T: Default> Default for Repl<'_, T> {
    #[inline]
    fn default() -> Self {
        Self {
            data: Default::default(),
            prompt: ">>>",
            exit: "exit",
            exit_message: "repl terminated",
            functions: HashMap::new(),
        }
    }
}

impl<'a, T> Repl<'a, T> {
    /// builds a new Repl from the given data
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            data,
            prompt: ">>>",
            exit: "exit",
            exit_message: "repl terminated",
            functions: HashMap::new(),
        }
    }
    /// same as take_arg, but returns the argument instead of storing it in self.argument
    pub fn get_input(&self) -> io::Result<Vec<String>> {
        print!("{}", &self.prompt);
        // flushing stdout because print! does'nt do it by default
        io::stdout()
            .flush()
            .expect("something went wrong when flushing the buffer");
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        // trimming \r\n and \n from the input
        buf = buf
            .trim_end_matches('\n')
            .trim_end_matches('\r')
            .to_string();
        let mut args = Vec::new();
        for i in buf.split_ascii_whitespace() {
            args.push(i.to_string());
        }
        Ok(args)
    }

    /// builder style menthod for adding a function.
    /// this function is chainable, use add_function if you don't want it to be chainable.
    #[inline]
    pub fn with_function(mut self, name: &'a str, func: fn(&mut T, Vec<String>)) -> Self {
        self.add_function(name, func);
        self
    }
    #[inline]
    pub fn with_data(mut self, data: T) -> Self {
        self.set_data(data);
        self
    }
    /// builder style method for changing the prompt.
    /// this function is chainable, use set_prompt if you don't want it to be chainable.
    #[inline]
    pub fn with_prompt(mut self, prompt: &'a str) -> Self {
        self.set_prompt(prompt);
        self
    }
    /// builder style method for changing the exit command.
    /// this function is chainable, use set_exit_command if you don't want it to be chainable.
    #[inline]
    pub fn with_exit_command(mut self, exit: &'a str) -> Self {
        self.set_exit_command(exit);
        self
    }
    /// builder style method for changing the exit message.
    /// this function is chainable, use set_exit_message if you don't want it to be chainable.
    #[inline]
    pub fn with_exit_message(mut self, exit_message: &'a str) -> Self {
        self.set_exit_message(exit_message);
        self
    }
    /// adds function to Repl, NOT chainable, use with_function if you want chaining
    #[inline]
    pub fn add_function(&mut self, name: &'a str, func: fn(&mut T, Vec<String>)) {
        self.functions.insert(name, utils::FnPtr(func));
    }
    #[inline]
    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
    /// sets prompt to argument, NOT chainable, use with_prompt if you want chaining.
    #[inline]
    pub fn set_prompt(&mut self, prompt: &'a str) {
        self.prompt = prompt;
    }
    /// sets exit command to argument, NOT chainable, use with_exit_command if you want chaining.
    #[inline]
    pub fn set_exit_command(&mut self, exit: &'a str) {
        self.exit = exit;
    }
    /// sets exit message to argument, NOT chainable, use with_exit_message if you want chaining.
    #[inline]
    pub fn set_exit_message(&mut self, exit_message: &'a str) {
        self.exit_message = exit_message;
    }
    /// runs the repl
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            let arguments = self.get_input()?;
            if self.functions.contains_key("") {
                self.functions[""].0(&mut self.data, arguments[0..arguments.len()].to_vec());
            }
            if self.functions.contains_key(arguments[0].as_str()) {
                self.functions[arguments[0].as_str()].0(
                    &mut self.data,
                    arguments[1..arguments.len()].to_vec(),
                );
            } else if arguments.concat() == self.exit {
                println!("{}", self.exit_message);
                break;
            }
        }
        Ok(())
    }
}
