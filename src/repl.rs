#![warn(clippy::all)]
use crate::utils::{self, parse, FnPtr};
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
    /// text displayed when repl gets an empty line as argument
    empty_argument_message: &'a str,
    /// text displayed when repl gets an unknown command as argument
    unknown_command_message: &'a str,
    /// hashmap that stores functions
    functions: HashMap<&'a str, FnPtr<T>>,
    /// parser function for input
    parser_fn: fn(String) -> Vec<String>,
}
impl<T: Default> Default for Repl<'_, T> {
    /// builds a new Repl from the given data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default();
    /// ```
    #[inline]
    fn default() -> Self {
        Repl::new(Default::default())
    }
}

impl<'a, T> Repl<'a, T> {
    /// builds a new Repl from the given data
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::new(());
    /// ```
    #[inline]
    pub fn new_with_depreciated_parser(data: T) -> Self {
        Self {
            data,
            prompt: ">>>",
            exit: "exit",
            exit_message: "repl terminated",
            functions: HashMap::new(),
            empty_argument_message: "",
            unknown_command_message: "",
            parser_fn: |buf| {
                buf.trim_end_matches('\n')
                    .trim_end_matches('\r')
                    .split_ascii_whitespace()
                    .map(|f| f.to_owned())
                    .collect()
            },
        }
    }
    #[inline]
    pub fn new(data: T) -> Self {
        Self {
            data,
            prompt: ">>>",
            exit: "exit",
            exit_message: "repl terminated",
            functions: HashMap::new(),
            empty_argument_message: "",
            unknown_command_message: "",
            parser_fn: parse,
        }
    }
    /// same as `take_arg`, but returns the argument instead of storing it in self.argument
    ///
    /// # Errors
    /// this function returns an error if
    /// - reading from stdin fails
    /// - flushing stdout fails
    pub fn get_input(&self) -> io::Result<Vec<String>> {
        print!("{}", &self.prompt);
        // flushing stdout because print! doesn't do it by default
        io::stdout().flush()?;
        let mut buf = String::new();
        io::stdin().read_line(&mut buf)?;
        Ok((self.parser_fn)(buf.trim().to_owned()))
    }

    /// builder style method for adding a function.
    /// this function is chainable, use `add_function` if you don't want it to be chainable.
    ///
    /// # Example
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_function("hello", hello);
    ///
    /// fn hello(_: &mut (), _: Vec<String>) {
    ///     println!("hello");
    /// }
    /// ```
    #[inline]
    pub fn with_function(mut self, name: &'a str, func: fn(&mut T, Vec<String>)) -> Self {
        self.add_function(name, func);
        self
    }
    /// builder style method for changing the parser.
    /// this function is chainable, use `set_parser` if you don't want it to be chainable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_parser(|raw| vec![raw]);
    /// ```
    #[inline]
    pub fn with_parser(mut self, parser: fn(String) -> Vec<String>) -> Self {
        self.set_parser(parser);
        self
    }
    /// sets parser function to argument, NOT chainable, use `with_parser` if you want chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<i32> = Repl::default();
    /// repl.set_parser(|raw| vec![raw]);
    /// ```
    #[inline]
    pub fn set_parser(&mut self, parser: fn(String) -> Vec<String>) {
        self.parser_fn = parser;
    }

    /// builder style method for changing the data.
    /// this function is chainable, use `set_data` if you don't want it to be chainable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_data(());
    /// ```
    #[inline]
    pub fn with_data(mut self, data: T) -> Self {
        self.set_data(data);
        self
    }
    /// builder style method for changing the prompt.
    /// this function is chainable, use `set_prompt` if you don't want it to be chainable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_prompt("+>");
    /// // the repl will display +> after every message now, instead of the default ">>>"
    /// ```
    #[inline]
    pub fn with_prompt(mut self, prompt: &'a str) -> Self {
        self.set_prompt(prompt);
        self
    }
    /// builder style method for changing the exit command.
    /// this function is chainable, use `set_exit_command` if you don't want it to be chainable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_exit_command("close");
    /// // the repl will close if you give "close" as input now
    /// ```
    #[inline]
    pub fn with_exit_command(mut self, exit: &'a str) -> Self {
        self.set_exit_command(exit);
        self
    }
    /// builder style method for changing the exit message.
    /// this function is chainable, use `set_exit_message` if you don't want it to be chainable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_exit_message("repl closed!");
    /// // the repl will display "repl closed!" on termination
    /// ```
    #[inline]
    pub fn with_exit_message(mut self, exit_message: &'a str) -> Self {
        self.set_exit_message(exit_message);
        self
    }
    /// builder style method for changing the exit message.
    /// this function is chainable, use `set_exit_message` if you don't want it to be chainable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let repl: Repl<()> = Repl::default().with_empty_argument_message("empty arg :(");
    /// // the repl will display "empty arg :(" on termination
    /// ```
    #[inline]
    pub fn with_empty_argument_message(mut self, empty_argument_message: &'a str) -> Self {
        self.set_empty_argument_message(empty_argument_message);
        self
    }
    /// adds function to Repl, not chainable, use `with_function` if you want chaining
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<()> = Repl::default();
    /// repl.add_function("hello", hello);
    /// fn hello(_: &mut (), _: Vec<String>) {
    ///     println!("Hello!")
    /// }
    /// ```
    #[inline]
    pub fn add_function(&mut self, name: &'a str, func: fn(&mut T, Vec<String>)) {
        self.functions.insert(name, utils::FnPtr(func));
    }
    /// sets data to argument, NOT chainable, use `with_data` if you want chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<i32> = Repl::default();
    /// repl.set_data(100);
    /// ```
    #[inline]
    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }
    /// sets prompt to argument, NOT chainable, use `with_prompt` if you want chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<()> = Repl::default();
    /// repl.set_prompt(":>");
    ///
    /// ```
    #[inline]
    pub fn set_prompt(&mut self, prompt: &'a str) {
        self.prompt = prompt;
    }
    /// sets exit command to argument, NOT chainable, use `with_exit_command` if you want chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<()> = Repl::default();
    /// repl.set_exit_command("close!");
    /// ```
    #[inline]
    pub fn set_exit_command(&mut self, exit: &'a str) {
        self.exit = exit;
    }
    /// sets exit message to argument, NOT chainable, use `with_exit_message` if you want chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<()> = Repl::default();
    /// repl.set_exit_message("bye!");
    /// ```
    #[inline]
    pub fn set_exit_message(&mut self, exit_message: &'a str) {
        self.exit_message = exit_message;
    }
    /// sets exit message to argument, NOT chainable, use `with_exit_message` if you want chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use repl_framework::Repl;
    /// let mut repl: Repl<()> = Repl::default();
    /// repl.set_empty_argument_message("empty argument list!");
    /// ```
    #[inline]
    pub fn set_empty_argument_message(&mut self, empty_argument_message: &'a str) {
        self.empty_argument_message = empty_argument_message;
    }
    /// runs the repl
    /// functions which have command `""` will be called if none of the other commands are not called.
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use repl_framework::Repl;
    /// Repl::default().with_function("", |_: &mut (), b| println!("{:?}", b)).run();
    /// ```
    ///
    /// # Errors
    /// - reading from stdin fails
    /// - flushing stdout fails
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            let arguments = self.get_input()?;
            if arguments.is_empty() {
                println!("{}", self.empty_argument_message);
            } else if arguments.concat() == self.exit {
                println!("{}", self.exit_message);
                break;
            } else if self.functions.contains_key(arguments[0].as_str()) {
                self.functions[arguments[0].as_str()].0(
                    &mut self.data,
                    arguments[1..arguments.len()].to_vec(),
                );
            } else if self.functions.contains_key("") {
                self.functions[""].0(&mut self.data, arguments[0..arguments.len()].to_vec());
            } else {
                println!("{}", self.unknown_command_message);
            }
        }
        Ok(())
    }
}
