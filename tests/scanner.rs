extern crate protoparse;

use protoparse::scanner::{Scanner, Token};

#[test]
fn should_return_eof_for_empty_input() {
    let input = "".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Ok(Token::EOF));

    let input2 = "  \t".to_string();
    let mut scanner2 = Scanner::new(&input2);
    assert_eq!(scanner2.next_token(), Ok(Token::EOF));
}

#[test]
fn should_scan_syntax_stmt() {
    let input = "syntax = \"proto3\";".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Ok(Token::Syntax));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::StrLit("proto3".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::EOF));
}

#[test]
fn should_scan_import_stmt() {
    let input = "import public \"other.proto\";".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Ok(Token::Import));
    assert_eq!(scanner.next_token(), Ok(Token::Public));
    assert_eq!(scanner.next_token(), Ok(Token::StrLit("other.proto".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::EOF));

    let input2 = "import weak \"other.proto\";".to_string();
    let mut scanner2 = Scanner::new(&input2);
    assert_eq!(scanner2.next_token(), Ok(Token::Import));
    assert_eq!(scanner2.next_token(), Ok(Token::Weak));
    assert_eq!(scanner2.next_token(), Ok(Token::StrLit("other.proto".to_string())));
    assert_eq!(scanner2.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner2.next_token(), Ok(Token::EOF));
}

#[test]
fn should_scan_package_stmt() {
    let input = "package foo.bar;".to_string();
    let mut scanner = Scanner::new(&input);

    assert_eq!(scanner.next_token(), Ok(Token::Package));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("foo".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Dot));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("bar".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::EOF));
}

#[test]
fn should_can_simple_option() {
    let input = "option java_package = \"com.example.foo\";".to_string();
    let mut scanner = Scanner::new(&input);

    assert_eq!(scanner.next_token(), Ok(Token::Option));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("java_package".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::StrLit("com.example.foo".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::EOF));
}

#[test]
fn should_scan_enum() {
    let input = "enum EnumAllowingAlias { option allow_alias = true; UNKNOWN = 0; STARTED = 1; RUNNING = 2 [(custom_option) = \"hello world\"]}".to_string();
    let mut scanner = Scanner::new(&input);

    assert_eq!(scanner.next_token(), Ok(Token::Enum));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("EnumAllowingAlias".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::LCurly));
    assert_eq!(scanner.next_token(), Ok(Token::Option));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("allow_alias".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::BoolLit(true)));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("UNKNOWN".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::DecimalLit(0)));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("STARTED".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::DecimalLit(1)));
    assert_eq!(scanner.next_token(), Ok(Token::Semicolon));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("RUNNING".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::DecimalLit(2)));
    assert_eq!(scanner.next_token(), Ok(Token::LBracket));
    assert_eq!(scanner.next_token(), Ok(Token::LParen));
    assert_eq!(scanner.next_token(), Ok(Token::Ident("custom_option".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::RParen));
    assert_eq!(scanner.next_token(), Ok(Token::Eq));
    assert_eq!(scanner.next_token(), Ok(Token::StrLit("hello world".to_string())));
    assert_eq!(scanner.next_token(), Ok(Token::RBracket));
    assert_eq!(scanner.next_token(), Ok(Token::RCurly));
    assert_eq!(scanner.next_token(), Ok(Token::EOF));
}

// TODO add message + service definition examples as test

// str literal tests

#[test]
fn should_parse_str_literal() {
    let input = "\"a string{})();,.\"".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Ok(Token::StrLit("a string{})();,.".to_string())));
}

#[test]
fn should_parse_str_literal_with_escaping() {
    let input = "\"string{with};.\\\"es(c)aping\\\".\"".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Ok(Token::StrLit("string{with};.\"es(c)aping\".".to_string())));
}

#[test]
fn should_return_err_on_unknown_escape() {
    let input = "\"\\k\"".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Err("Lexical error: unknown escaping"));
}

#[test]
fn should_return_err_on_non_closed_str_literal() {
    let input = "\"foo message".to_string();
    let mut scanner = Scanner::new(&input);
    assert_eq!(scanner.next_token(), Err("Lexical error: unclosed string literal"));
}
