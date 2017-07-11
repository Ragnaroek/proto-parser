use super::scanner::Scanner;
use super::ast::{ProtoDef, Syntax};


//TODO Impl recursive descend parser here :)

pub fn parse(buffer: &String) -> Result<ProtoDef, &'static str> {
    let mut scanner = Scanner::new(buffer);
    let tk = scanner.next_token();

    println!("token: {:?}", tk);

    return Ok(ProtoDef{syntax: Syntax::V3});
}
