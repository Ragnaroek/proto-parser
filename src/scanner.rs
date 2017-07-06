
use std::str::Chars;
use std::iter::Peekable;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Syntax,
    Eq,
    StrLit(String),
    IntLit(u32),
    Import,
    Package,
    Ident(String),
    Service,
    LCurly,
    RCurly,
    Rpc,
    LParen,
    RParen,
    Returns,
    Semicolon,
    Dot,
    Message,
    //types
    TDouble,
    TFloat,
    TInt32,
    TInt64,
    TUint32,
    TUint64,
    TSint32,
    TSint64,
    TFixed32,
    TFixed64,
    TSfixed32,
    TSfixed64,
    TBool,
    TString,
    TBytes,
    EOF
}

//use rust_phf once feature(plugin) can be used in the non-nightly
lazy_static! {
    static ref IDENT_MAP: HashMap<&'static str, Token> = {
        let mut map = HashMap::new();
        map.insert("syntax".into(), Token::Syntax);
        map.insert("import".into(), Token::Import);
        map.insert("package".into(), Token::Package);
        map.insert("service".into(), Token::Service);
        map.insert("rpc".into(), Token::Rpc);
        map.insert("returns".into(), Token::Returns);
        map.insert("message".into(), Token::Message);

        map.insert("double".into(), Token::TDouble);
        map.insert("float".into(), Token::TFloat);
        map.insert("int32".into(), Token::TInt32);
        map.insert("int64".into(), Token::TInt64);
        map.insert("uint32".into(), Token::TUint32);
        map.insert("uint64".into(), Token::TUint64);
        map.insert("sint32".into(), Token::TSint32);
        map.insert("sint64".into(), Token::TSint64);
        map.insert("fixed32".into(), Token::TFixed32);
        map.insert("fixed64".into(), Token::TFixed64);
        map.insert("sfixed32".into(), Token::TSfixed32);
        map.insert("sfixed64".into(), Token::TSfixed64);
        map.insert("bool".into(), Token::TBool);
        map.insert("string".into(), Token::TString);
        map.insert("bytes".into(), Token::TBytes);
        map
    };
}


pub struct Scanner<'a> {
    buf: Peekable<Chars<'a>>
}

impl<'a> Scanner<'a> {

    pub fn new(buffer: &'a String) -> Scanner<'a> {
        return Scanner{buf: buffer.chars().peekable()};
    }

    pub fn next_token(&mut self) -> Token {
        self.unread_whitespace();

        //read token
        let mut token = String::new();
        loop {
            let peek = self.buf.peek().map(|c| *c);
            match peek {
                None => return Token::EOF,
                Some(c) => {
                    if c.is_whitespace() { // TODO test for chars not allowed in ident start: =, {, }, (, ), ;, .
                        break; //end of token
                    } else {
                        token.push(c);
                        self.buf.next();
                    }
                }
            }
        }

        let lookup_token = IDENT_MAP.get(&token[..]);
        if lookup_token.is_some() {
            return lookup_token.unwrap().clone();
        }
        return Token::Ident(token)
    }

    fn unread_whitespace(&mut self) {
        loop {
            let peek = self.buf.peek().map(|c| *c);
            match peek {
                None => return, //caller will detect EOF
                Some(c) => {
                    if !c.is_whitespace() {
                        return;
                    } else {
                        self.buf.next();
                    }
                }
            }
        }
    }
}
