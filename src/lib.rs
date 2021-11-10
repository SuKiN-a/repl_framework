//! repl_framework
//! a crate to create a simple toy repl
//!
//! # Examples
//!
//! ```rust, no_run
//! use repl_framework::Repl;
//! fn main() -> std::io::Result<()> {
//!     Repl::default()
//!         .with_function("", |data_store: &mut Vec<String>, data: Vec<String>| {
//!             data_store.extend(data)
//!         })
//!        .run()
//! }
//! ```
mod repl;

mod utils;
pub use repl::Repl;
