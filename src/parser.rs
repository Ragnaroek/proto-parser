use super::scanner::{Scanner, Token};
use super::ast::{ProtoDef, Syntax, Import};


//TODO Impl recursive descend parser here :)

pub fn parse(buffer: &String) -> Result<ProtoDef, &str> {
    let mut scanner = Scanner::new(buffer);

    let syn = parse_syntax(&mut scanner)?;
    let mut def = ProtoDef::new(syn);

    let mut lookahead = scanner.next_token()?;
    while lookahead != Token::EOF {
        match lookahead {
            // TODO parse also package, option, message, enum, service and emptyStmt
            Token::Import => {
                let imp = parse_import(&mut scanner)?;
                def.add_import(imp);
            },
            _ => return Err("unexpected token")
        }

        lookahead = scanner.next_token()?;
    }
    return Ok(def);
}

fn parse_syntax(scanner: &mut Scanner) -> Result<Syntax, &'static str> {
    expect(scanner, Token::Syntax)?;
    expect(scanner, Token::Eq)?;
    expect(scanner, Token::StrLit("proto3".to_string()))?;
    expect(scanner, Token::Semicolon)?;
    return Ok(Syntax::V3);
}

fn parse_import(scanner: &mut Scanner) -> Result<Import, &'static str> {
    //TODO impl import parsing!
    return Ok(Import{});
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
