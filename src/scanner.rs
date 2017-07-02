
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Syntax,
    Eq,
    StrLit(String),
    IntLit(u32),
    Import,
    Package,
    Ident,
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
                    if c.is_whitespace() { // TODO test for chars not allowed in ident
                        break; //end of token
                    } else {
                        token.push(c);
                        self.buf.next();
                    }
                }
            }
        }

        println!("#### token {:?}", token);

        //TODO: create sym-table str (static) -> Token, populate with keywords
        // lookup Token for detected token-string

        return Token::EOF;
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
