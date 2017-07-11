use super::scanner::{Scanner, Token};
use super::ast::{ProtoDef, Syntax};


//TODO Impl recursive descend parser here :)

pub fn parse(buffer: &String) -> Result<ProtoDef, &str> {
    let mut scanner = Scanner::new(buffer);

    syntax(&mut scanner)?;

    //TODO parse list of top-level statements!

    return Ok(ProtoDef{syntax: Syntax::V3});
}

fn syntax(scanner: &mut Scanner) -> Result<Syntax, &'static str> {
    expect(scanner, Token::Syntax)?;
    expect(scanner, Token::Eq)?;
    expect(scanner, Token::StrLit("proto3".to_string()))?;
    return Ok(Syntax::V3);
}

fn expect(mut scanner: &mut Scanner, expected: Token) -> Result<Token, &'static str> {
    let next = scanner.next_token();
    if let Err(e) = next {
        return Err(e);
    }

    let tk = next.unwrap();
    if tk != expected {
        return Err("unexpected token");
    }
    return Ok(tk);
}
