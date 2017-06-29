use std::path::Path;

use super::scanner::Scanner;

#[derive(Debug)]
pub struct ProtoDef {}

pub fn parse(path: &Path) -> ProtoDef {

    let scanner = Scanner::new(path);
    let tk = scanner.next_token();

    println!("token: {:?}", tk);

    return ProtoDef{};
}
