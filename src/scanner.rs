
use std::str::Chars;

#[derive(Debug)]
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
    buf: Chars<'a>
}

impl<'a> Scanner<'a> {

    pub fn new(buffer: &'a String) -> Scanner<'a> {
        return Scanner{buf: buffer.chars()};
    }

    pub fn next_token(&mut self) -> Token {
        println!("### {:?}", self.buf.next());

        //TODO: create sym-table str -> Token, populate with keywords
        // find next word

        return Token::EOF;
    }
}
