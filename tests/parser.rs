extern crate protoparse;

use protoparse::parser::{parse};
use protoparse::ast::{Syntax, ImportType};

#[test]
fn should_parse_syntax() {
    let input = "syntax = \"proto3\";".to_string();
    let result = parse(&input).unwrap();

    assert_eq!(result.syntax, Syntax::V3);
}

#[test]
fn should_parse_unmodified_import() {
    let input = min_file() + "import \"foobar.proto\"";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Default);
    assert_eq!(result.imports[0].name, "foobar.proto".to_string());
}

#[test]
fn should_parse_weak_import() {
    let input = min_file() + "import weak \"imp.proto\"";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Weak);
    assert_eq!(result.imports[0].name, "imp.proto".to_string());
}

#[test]
fn should_parse_public_import() {
    let input = min_file() + "import public \"header.proto\"";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Public);
    assert_eq!(result.imports[0].name, "header.proto".to_string());
}

#[test]
fn should_parse_empty_stmts() {
    let input = min_file() + ";;;;";
    parse(&input).unwrap();
}

// helper methods

fn min_file() -> String {
    return "syntax = \"proto3\";".to_string();
}
