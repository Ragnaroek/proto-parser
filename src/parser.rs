use super::scanner::{Scanner, Token};
use super::ast::*;

pub fn parse(buffer: &String) -> Result<ProtoDef, &str> {
    let mut scanner = Scanner::new(buffer);

    let syn = parse_syntax(&mut scanner)?;
    let mut def = ProtoDef::new(syn);

    let mut lookahead = scanner.next_token()?;
    while lookahead != Token::EOF {
        match lookahead {
            //TODO parse also service, message, enum
            Token::Import => {
                let imp = parse_import(&mut scanner)?;
                def.add_import(imp);
            },
            Token::Package => {
                let pckg = parse_package(&mut scanner)?;
                def.add_package(pckg);
            },
            Token::Option => {
                let opt = parse_option(&mut scanner)?;
                def.add_option(opt);
            },
            Token::Service => {
                let service = parse_service(&mut scanner)?;
                def.add_service(service);
            },
            Token::Semicolon => {} //simply ignore that
            token => {
                println!("unexpected token: {:?}", token);
                return Err("unexpected token");
            }
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

    expect(scanner, Token::Semicolon)?;

    return Ok(Import{import_type, name});
}

fn parse_package(scanner: &mut Scanner) -> Result<Package, &'static str> {
    let full_ident = parse_full_ident(scanner, Token::Semicolon)?;
    return Ok(Package{full_ident: full_ident});
}

//term_token is consumed
fn parse_full_ident(scanner: &mut Scanner, term_token: Token) -> Result<FullIdent, &'static str> {

    let mut idents = Vec::new();
    let mut rparen = false;

    let mut first_ident = scanner.next_token()?;
    match first_ident {
        Token::Ident(s) => idents.push(s),
        Token::LParen => {
            first_ident = scanner.next_token()?;
            rparen = true;
            match first_ident {
                Token::Ident(s) => idents.push(s),
                _ => return Err("FullIden   t: identifier expected")
            }
        }
        _ => return Err("FullIdent: identifier expected")
    }

    let mut ident_term = term_token.clone();
    if rparen {
        ident_term = Token::RParen;
    }

    let mut next = scanner.next_token()?;
    loop {
        if next == ident_term {
            break;
        }

        if next != Token::Dot {
            return Err("FullIdent: . expected");
        }
        next = scanner.next_token()?;
        match next {
            Token::Ident(s) => idents.push(s),
            _ => return Err("FullIdent: identifier expected")
        }
        next = scanner.next_token()?;
    }

    if rparen {
        expect(scanner, term_token)?;
    }

    return Ok(FullIdent{idents: idents});
}

fn parse_option(scanner: &mut Scanner) -> Result<ProtoOption, &'static str> {

    let ident = parse_full_ident(scanner, Token::Eq)?;
    let constant = parse_constant(scanner)?;

    return Ok(ProtoOption{
        full_ident: ident,
        constant: constant
    });
}

//also parses the semicolon!
fn parse_constant(scanner: &mut Scanner) -> Result<ConstantValue, &'static str> {
    let next = scanner.next_token()?;
    match next {
        Token::StrLit(s) => {
            expect(scanner, Token::Semicolon)?;
            return Ok(ConstantValue::StringValue(s));
        },
        Token::DecimalLit(d) => {
            expect(scanner, Token::Semicolon)?;
            return Ok(ConstantValue::NumberValue(d as f32));
        },
        Token::BoolLit(b) => {
            expect(scanner, Token::Semicolon)?;
            return Ok(ConstantValue::BoolValue(b));
        },
        Token::Ident(n) => {
            let ident_next = scanner.next_token()?;
            if ident_next == Token::Semicolon {
                return Ok(ConstantValue::IdentValue(FullIdent::new(vec![n])));
            } else if ident_next == Token::Dot {
                let mut rest_ident = parse_full_ident(scanner, Token::Semicolon)?;
                rest_ident.insert(0, n);
                return Ok(ConstantValue::IdentValue(rest_ident));
            }
            return Err("unexepected token in constant identifier")
        },
        Token::Plus => {
            let num = scanner.next_token()?;
            match num {
                Token::DecimalLit(d) => return Ok(ConstantValue::NumberValue(d as f32)),
                _ => Err("unexpected token after +")
            }
        },
        Token::Minus => {
            let num = scanner.next_token()?;
            match num {
                Token::DecimalLit(d) => return Ok(ConstantValue::NumberValue(-(d as f32))),
                _ => Err("unexpected token after -")
            }
        },
        _ => Err("unexpected token in constant expression")
    }

    // TODO parse +, - num values
}

fn parse_service(scanner: &mut Scanner) -> Result<Service, &'static str> {

    let name = expect_ident(scanner)?;
    expect(scanner, Token::LCurly)?;

    let mut rpcs = Vec::new();
    let mut next = scanner.next_token()?;
    while next == Token::Rpc {
        let rpc_def = parse_rpc(scanner)?;
        rpcs.push(rpc_def);
        next = scanner.next_token()?;
    }

    if next != Token::RCurly {
        return Err("Unexpected token, expected }");
    }

    return Ok(Service{name: name, rpcs});
}

fn parse_rpc(scanner: &mut Scanner) -> Result<Rpc, &'static str> {
    let name = expect_ident(scanner)?;

    expect(scanner, Token::LParen)?;
    let req_message_type = parse_full_ident(scanner, Token::RParen)?;
    expect(scanner, Token::Returns)?;

    expect(scanner, Token::LParen)?;
    let resp_message_type = parse_full_ident(scanner, Token::RParen)?;
    expect(scanner, Token::Semicolon)?;

    return Ok(Rpc{name, request_type: req_message_type, response_type: resp_message_type });
}

fn expect_ident(scanner: &mut Scanner) -> Result<String, &'static str> {
    let next = scanner.next_token()?;
    match next {
        Token::Ident(name) => Ok(name),
        _ => Err("Ident expected")
    }
}

fn expect(scanner: &mut Scanner, expected: Token) -> Result<Token, &'static str> {
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
