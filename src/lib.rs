#![crate_name = "protoparser"]
#![crate_type = "lib"]

#[macro_use]
extern crate lazy_static;

pub mod parser;
pub mod scanner;
pub mod ast;
pub mod error;
