use super::scanner::Scanner;

#[derive(Debug)]
pub struct ProtoDef {}

pub fn parse(buffer: &String) -> ProtoDef {
    let mut scanner = Scanner::new(buffer);
    let tk = scanner.next_token();

    println!("token: {:?}", tk);

    return ProtoDef{};
}
