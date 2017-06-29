
use std::path::Path;
use std::io::Read;
use std::fs::File;

#[derive(Debug)]
pub enum Token {
    Syntax,
    Eq,
    StrLit(String),
    EOF
}

pub struct Scanner {
    buf: String
}

impl Scanner {

    pub fn new(path: &Path) -> Scanner {
        let mut buffer = String::new();
        File::open(path).unwrap().read_to_string(&mut buffer).unwrap();
        return Scanner{buf: buffer};
    }

    pub fn next_token(&self) -> Token {
        return Token::EOF;
    }
}
