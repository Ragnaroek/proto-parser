
use std::path::Path;
use std::io::Read;
use std::fs::File;

pub enum Token {
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

    pub fn next_token() -> Token {
        return Token::EOF;
    }
}
