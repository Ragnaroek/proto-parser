use super::scanner::{Scanner, Token};
use super::ast::{ProtoDef, Syntax, Import, ImportType};

pub fn parse(buffer: &String) -> Result<ProtoDef, &str> {
    let mut scanner = Scanner::new(buffer);

    let syn = parse_syntax(&mut scanner)?;
    let mut def = ProtoDef::new(syn);

    let mut lookahead = scanner.next_token()?;
    while lookahead != Token::EOF {
        match lookahead {
            // TODO parse also package, option, message, enum, service
            Token::Import => {
                let imp = parse_import(&mut scanner)?;
                def.add_import(imp);
            },
            Token::Semicolon => {} //simply ignore that
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

    let mut next = scanner.next_token()?;
    let mut import_type = ImportType::Default;

    if next == Token::Weak {
        import_type = ImportType::Weak;
        next = scanner.next_token()?;
    } else if next == Token::Public {
        import_type = ImportType::Public;
        next = scanner.next_token()?;
    }

    let name = match next {
        Token::StrLit(s) => s,
        _ => return Err("string literal expected in import")
    };

    return Ok(Import{import_type, name});
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
