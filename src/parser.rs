use std::fs::File;
use std::io::Read;
use std::path::Path;

use super::ast::*;
use super::error::{ProtoParseError, err};
use super::scanner::{Scanner, Token};

pub fn parse(buffer: &String) -> Result<ProtoDef, ProtoParseError> {
    let mut scanner = Scanner::new(buffer);

    let syn = parse_syntax(&mut scanner)?;
    let mut def = ProtoDef::new(syn);

    let mut lookahead = scanner.next_token()?;
    while lookahead != Token::EOF {
        match lookahead {
            //TODO parse also message, enum
            Token::Import => {
                let imp = parse_import(&mut scanner)?;
                def.add_import(imp);
            }
            Token::Package => {
                let pckg = parse_package(&mut scanner)?;
                def.add_package(pckg);
            }
            Token::Option => {
                let opt = parse_option(&mut scanner)?;
                def.add_option(opt);
            }
            Token::Service => {
                let service = parse_service(&mut scanner)?;
                def.add_service(service);
            }
            Token::Message => {
                let message = parse_message(&mut scanner)?;
                def.add_message(message);
            }
            Token::Semicolon => {} //simply ignore that
            token => {
                return err(&format!("unexpected token {:?}", token));
            }
        }

        lookahead = scanner.next_token()?;
    }
    return Ok(def);
}

pub fn parse_from_file(file: &Path) -> Result<ProtoDef, ProtoParseError> {
    let mut buffer = String::new();
    let open_result = File::open(file);
    if open_result.is_err() {
        return err("proto file cannot be opened");
    }
    let read_result = open_result.unwrap().read_to_string(&mut buffer);
    if read_result.is_err() {
        return err(&format!("cannot read file: {:?}", file));
    }
    //TODO handle error and return parse_error
    return parse(&buffer);
}

fn parse_syntax(scanner: &mut Scanner) -> Result<Syntax, ProtoParseError> {
    expect(scanner, Token::Syntax)?;
    expect(scanner, Token::Eq)?;
    expect(scanner, Token::StrLit("proto3".to_string()))?;
    expect(scanner, Token::Semicolon)?;
    return Ok(Syntax::V3);
}

fn parse_import(scanner: &mut Scanner) -> Result<Import, ProtoParseError> {
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
        _ => return err("string literal expected in import"),
    };

    expect(scanner, Token::Semicolon)?;

    return Ok(Import { import_type, name });
}

fn parse_package(scanner: &mut Scanner) -> Result<Package, ProtoParseError> {
    let full_ident = parse_full_ident(scanner, Token::Semicolon)?;
    return Ok(Package {
        full_ident: full_ident,
    });
}

//term_token is consumed
fn parse_full_ident(
    scanner: &mut Scanner,
    term_token: Token,
) -> Result<FullIdent, ProtoParseError> {
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
                _ => return err("FullIdent: identifier expected"),
            }
        }
        _ => return err("FullIdent: identifier expected"),
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
            return err("FullIdent: . expected");
        }
        next = scanner.next_token()?;
        match next {
            Token::Ident(s) => idents.push(s),
            _ => return err("FullIdent: identifier expected"),
        }
        next = scanner.next_token()?;
    }

    if rparen {
        expect(scanner, term_token)?;
    }

    return Ok(FullIdent { idents: idents });
}

fn parse_option(scanner: &mut Scanner) -> Result<ProtoOption, ProtoParseError> {
    let ident = parse_full_ident(scanner, Token::Eq)?;
    let constant = parse_constant(scanner)?;

    return Ok(ProtoOption {
        full_ident: ident,
        constant: constant,
    });
}

//also parses the semicolon!
fn parse_constant(scanner: &mut Scanner) -> Result<ConstantValue, ProtoParseError> {
    let next = scanner.next_token()?;
    match next {
        Token::StrLit(s) => {
            expect(scanner, Token::Semicolon)?;
            return Ok(ConstantValue::StringValue(s));
        }
        Token::DecimalLit(d) => {
            expect(scanner, Token::Semicolon)?;
            return Ok(ConstantValue::NumberValue(d as f32));
        }
        Token::BoolLit(b) => {
            expect(scanner, Token::Semicolon)?;
            return Ok(ConstantValue::BoolValue(b));
        }
        Token::Ident(n) => {
            let ident_next = scanner.next_token()?;
            if ident_next == Token::Semicolon {
                return Ok(ConstantValue::IdentValue(FullIdent::new(vec![n])));
            } else if ident_next == Token::Dot {
                let mut rest_ident = parse_full_ident(scanner, Token::Semicolon)?;
                rest_ident.insert(0, n);
                return Ok(ConstantValue::IdentValue(rest_ident));
            }
            return err("unexepected token in constant identifier");
        }
        Token::Plus => {
            let num = scanner.next_token()?;
            match num {
                Token::DecimalLit(d) => return Ok(ConstantValue::NumberValue(d as f32)),
                _ => err("unexpected token after +"),
            }
        }
        Token::Minus => {
            let num = scanner.next_token()?;
            match num {
                Token::DecimalLit(d) => return Ok(ConstantValue::NumberValue(-(d as f32))),
                _ => err("unexpected token after -"),
            }
        }
        _ => err("unexpected token in constant expression"),
    }

    // TODO parse +, - num values
}

fn parse_service(scanner: &mut Scanner) -> Result<Service, ProtoParseError> {
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
        return err("Unexpected token, expected }");
    }

    return Ok(Service { name: name, rpcs });
}

fn parse_rpc(scanner: &mut Scanner) -> Result<Rpc, ProtoParseError> {
    let name = expect_ident(scanner)?;

    expect(scanner, Token::LParen)?;
    let req_message_type = parse_full_ident(scanner, Token::RParen)?;
    expect(scanner, Token::Returns)?;

    expect(scanner, Token::LParen)?;
    let resp_message_type = parse_full_ident(scanner, Token::RParen)?;

    let next = scanner.next_token()?;
    if next == Token::LCurly {
        expect(scanner, Token::RCurly)?;
    } else if next == Token::Semicolon {
        //do nothing
    } else {
        return err("unexpected token, Semicolon or {} expected");
    }

    return Ok(Rpc {
        name,
        request_type: req_message_type,
        response_type: resp_message_type,
    });
}

fn parse_message(scanner: &mut Scanner) -> Result<Message, ProtoParseError> {
    let name = expect_ident(scanner)?;
    expect(scanner, Token::LCurly)?;
    let mut fields = Vec::new();
    let mut peeked = scanner.next_token()?;
    while peeked != Token::RCurly {
        let field = parse_field(peeked, scanner)?;
        fields.push(field);
        peeked = scanner.next_token()?;
    }
    return Ok(Message { name, fields });
}

fn parse_field(peeked: Token, scanner: &mut Scanner) -> Result<Field, ProtoParseError> {
    let mut next = peeked;
    let mut repeated = false;
    if next == Token::Repeated {
        repeated = true;
        next = scanner.next_token()?;
    }
    let field_type = is_type(next)?;
    let name = expect_keyword_ident(scanner)?;
    expect(scanner, Token::Eq)?;
    let field_number = expect_decimal_lit(scanner)?;
    expect(scanner, Token::Semicolon)?;
    return Ok(Field {
        name,
        field_type,
        repeated,
        field_number,
    });
}

fn is_type(token: Token) -> Result<Type, ProtoParseError> {
    match token {
        Token::TDouble => Ok(Type::Double),
        Token::TFloat => Ok(Type::Float),
        Token::TInt32 => Ok(Type::Int32),
        Token::TInt64 => Ok(Type::Int64),
        Token::TUint32 => Ok(Type::Uint32),
        Token::TUint64 => Ok(Type::Uint64),
        Token::TSint32 => Ok(Type::Sint32),
        Token::TSint64 => Ok(Type::Sint64),
        Token::TFixed32 => Ok(Type::Fixed32),
        Token::TFixed64 => Ok(Type::Fixed64),
        Token::TSfixed32 => Ok(Type::Sfixed32),
        Token::TSfixed64 => Ok(Type::Sfixed64),
        Token::TBool => Ok(Type::Bool),
        Token::TString => Ok(Type::String),
        Token::TBytes => Ok(Type::Bytes),
        _ => err(&format!("unexpected token {:?}, expected type", token)),
    }
}

fn expect_decimal_lit(scanner: &mut Scanner) -> Result<u32, ProtoParseError> {
    let next = scanner.next_token()?;
    match next {
        Token::DecimalLit(u) => Ok(u),
        _ => err(&format!("DecimalLiteral expected, got {:?}", next)),
    }
}

fn expect_ident(scanner: &mut Scanner) -> Result<String, ProtoParseError> {
    let next = scanner.next_token()?;
    match next {
        Token::Ident(name) => Ok(name),
        _ => err(&format!("Ident expected, got {:?}", next)),
    }
}

//a "real" ident, or a keyword that can be used as an ident
fn expect_keyword_ident(scanner: &mut Scanner) -> Result<String, ProtoParseError> {
    let next = scanner.next_token()?;
    match next {
        Token::Ident(name) => Ok(name),
        Token::Message => Ok("message".to_string()),
        //TODO more keywords are allowed as ident name, list them here
        _ => err(&format!("Ident expected, got {:?}", next)),
    }
}

fn expect(scanner: &mut Scanner, expected: Token) -> Result<Token, ProtoParseError> {
    let next = scanner.next_token()?;

    if next != expected {
        return err(&format!(
            "unexpected token, expected {:?} got {:?}",
            expected, next
        ));
    }
    return Ok(next);
}
