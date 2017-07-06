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
    let input = "syntax = \"proto3\";".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Token::Syntax);
    assert_eq!(scanner.next_token(), Token::Eq);
    assert_eq!(scanner.next_token(), Token::StrLit("proto3".to_string()));
    assert_eq!(scanner.next_token(), Token::Semicolon);
    assert_eq!(scanner.next_token(), Token::EOF);
}

// str literal test
#[test]
fn should_parse_str_literal() {
    let input = "\"a string{})();,.\"".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Token::StrLit("a string{})();,.".to_string()));
}

#[test]
fn should_parse_str_literal_with_escaping() {
    let input = "\"string{with};.\\\"es(c)aping\\\".\"".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Token::StrLit("string{with};.\"es(c)aping\".".to_string()));
}

// TODO Test non-closed str literal (should return scan_error)


// helper methods
