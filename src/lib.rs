#![crate_name = "protoparse"]
#![crate_type = "lib"]

#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod error;
pub mod parser;
pub mod scanner;
