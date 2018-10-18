
use std::str::Chars;
use std::iter::Peekable;
use std::collections::HashMap;
use std::result::Result;
use std::str;

use super::error::{err, ProtoParseError};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Syntax,
    Eq,
    StrLit(String),
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
    Comma,
    Message,
    Public,
    Weak,
    Option,
    Enum,
    BoolLit(bool),
    DecimalLit(u32),
    LBracket,
    RBracket,
    Lt,
    Gt,
    Map,
    Repeated,
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
    EOF,
    Plus,
    Minus
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
        map.insert("public".into(), Token::Public);
        map.insert("weak".into(), Token::Weak);
        map.insert("option".into(), Token::Option);
        map.insert("enum".into(), Token::Enum);
        map.insert("true".into(), Token::BoolLit(true));
        map.insert("false".into(), Token::BoolLit(false));
        map.insert("map".into(), Token::Map);
        map.insert("repeated".into(), Token::Repeated);

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

fn non_ident_char(c: char) -> bool {
    return c == '{' ||
           c == '}' ||
           c == '[' ||
           c == ']' ||
           c == '(' ||
           c == ')' ||
           c == '=' ||
           c == ',' ||
           c == ';' ||
           c == '<' ||
           c == '>' ||
           c == '.' ||
           c == '+' ||
           c == '-';
}

impl<'a> Scanner<'a> {

    pub fn new(buffer: &'a String) -> Scanner<'a> {
        return Scanner{buf: buffer.chars().peekable()};
    }

    pub fn next_token(&mut self) -> Result<Token, ProtoParseError> {
        self.unread_whitespace();

        let mut token = String::new();
        let mut str_lit = false;
        let mut dec_lit = false;
        let mut escaped = false;
        loop {
            let peek = self.buf.peek().map(|c| *c);
            match peek {
                None => {
                    if str_lit {
                        return err("Lexical error: unclosed string literal");
                    }
                    return Ok(Token::EOF)
                },
                Some(c) => {
                    if str_lit {
                        match c {
                            // TODO there are more escaping chars in proto
                            '"'  => {
                                if escaped {
                                    self.buf.next(); token.push(c); escaped = false;
                                } else {
                                    self.buf.next(); break;
                                }
                            }
                            '\\' => {
                                if escaped {
                                    self.buf.next(); token.push(c); escaped = false;
                                } else {
                                    escaped = true; self.buf.next();
                                }
                            }
                            _ => {
                                if escaped {
                                    return err("Lexical error: unknown escaping");
                                }
                                token.push(c); self.buf.next();
                            }
                        }
                    } else {
                        if token.len() == 0 {
                            match c {
                                '/' => {self.unread_line_comment(); continue},
                                '{' => {self.buf.next(); return Ok(Token::LCurly)},
                                '}' => {self.buf.next(); return Ok(Token::RCurly)},
                                '=' => {self.buf.next(); return Ok(Token::Eq)},
                                '(' => {self.buf.next(); return Ok(Token::LParen)},
                                ')' => {self.buf.next(); return Ok(Token::RParen)},
                                ',' => {self.buf.next(); return Ok(Token::Comma)},
                                ';' => {self.buf.next(); return Ok(Token::Semicolon)},
                                '.' => {self.buf.next(); return Ok(Token::Dot)},
                                '[' => {self.buf.next(); return Ok(Token::LBracket)},
                                ']' => {self.buf.next(); return Ok(Token::RBracket)},
                                '<' => {self.buf.next(); return Ok(Token::Lt)},
                                '>' => {self.buf.next(); return Ok(Token::Gt)},
                                '+' => {self.buf.next(); return Ok(Token::Plus)},
                                '-' => {self.buf.next(); return Ok(Token::Minus)},
                                '"' => {str_lit = true; self.buf.next(); continue},
                                ch if ch.is_digit(10) => {dec_lit = true; token.push(c); self.buf.next();},
                                _ => {
                                    token.push(c);
                                    self.buf.next();
                                }
                            }
                        } else {
                            if c.is_whitespace() || non_ident_char(c) {
                                break;
                            } else {
                                token.push(c);
                                self.buf.next();
                            }
                        }
                    }
                }
            }
        }

        if dec_lit {
            let dec = token.parse::<u32>();
            match dec {
                Err(_) => return err("Lexical error: illegal decimal literal"),
                Ok(n) => return Ok(Token::DecimalLit(n))
            }
        }

        if str_lit {
            return Ok(Token::StrLit(token));
        }

        let lookup_token = IDENT_MAP.get(&token[..]);
        if lookup_token.is_some() {
            return Ok(lookup_token.unwrap().clone());
        }
        return Ok(Token::Ident(token))
    }

    fn unread_line_comment(&mut self) {
        loop {
            let peek = self.buf.peek().map(|c| *c);
            match peek {
                None => return,
                Some(c) => {
                    self.buf.next();
                    if c == '\n' {
                        self.unread_whitespace();
                        return;
                    }
                }
            }
        }
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
