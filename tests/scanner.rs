extern crate protoparse;

use protoparse::scanner::{Scanner, Token};

// TODO Input "" should return EOF

#[test]
fn should_return_eof_for_empty_input() {
    let input = "".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Token::EOF);

    let input2 = "  \t".to_string();
    let mut scanner2 = Scanner::new(&input2);
    assert_eq!(scanner2.next_token(), Token::EOF);
}

#[test]
fn should_scan_syntax_stmt() {
    let input = "syntax = \"proto3\";" .to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Token::Syntax);
    assert_eq!(scanner.next_token(), Token::Eq);
    assert_eq!(scanner.next_token(), Token::StrLit("proto3".to_string()));
    assert_eq!(scanner.next_token(), Token::Semicolon);
    assert_eq!(scanner.next_token(), Token::EOF);
}

// helper methods
