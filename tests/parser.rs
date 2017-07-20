extern crate protoparse;

use protoparse::parser::{parse};
use protoparse::ast::*;

#[test]
fn should_parse_syntax() {
    let input = "syntax = \"proto3\";".to_string();
    let result = parse(&input).unwrap();

    assert_eq!(result.syntax, Syntax::V3);
}

#[test]
fn should_parse_unmodified_import() {
    let input = min_file() + "import \"foobar.proto\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Default);
    assert_eq!(result.imports[0].name, "foobar.proto".to_string());
}

#[test]
fn should_parse_weak_import() {
    let input = min_file() + "import weak \"imp.proto\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.imports.len(), 1);
    assert_eq!(result.imports[0].import_type, ImportType::Weak);
    assert_eq!(result.imports[0].name, "imp.proto".to_string());
}

#[test]
fn should_parse_public_import() {
    let input = min_file() + "import public \"header.proto\";";
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

#[test]
fn parse_package_with_simple_name() {
    let input = min_file() + "package foo;";
    let result = parse(&input).unwrap();

    assert_eq!(result.packages.len(), 1);
    assert_eq!(result.packages[0].full_ident.idents.len(), 1);
    assert_eq!(result.packages[0].full_ident.idents[0], "foo".to_string());
}

#[test]
fn parse_package_with_full_name() {
    let input = min_file() + "package foo.bar.baz;";
    let result = parse(&input).unwrap();

    assert_eq!(result.packages.len(), 1);
    assert_eq!(result.packages[0].full_ident.idents.len(), 3);
    assert_eq!(result.packages[0].full_ident.idents[0], "foo".to_string());
    assert_eq!(result.packages[0].full_ident.idents[1], "bar".to_string());
    assert_eq!(result.packages[0].full_ident.idents[2], "baz".to_string());
}

#[test]
fn parse_not_package_without_name() {
    let input = min_file() + "package;";
    assert!(parse(&input).is_err());
}

#[test]
fn parse_option_with_simple_ident_and_number_value() {
    let input = min_file() + "option my_option = 42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].full_ident.idents.len(), 1);
    assert_eq!(result.options[0].full_ident.idents[0], "my_option".to_string());
    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_string_value() {
    let input = min_file() + "option my_option = \"strOpt\";";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::StringValue("strOpt".to_string()));
}

#[test]
fn parse_option_bool_value() {
    let input = min_file() + "option my_option = true;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::BoolValue(true));
}

#[test]
fn parse_option_full_ident_value() {
    let input = min_file() + "option my_option = my.ident.value;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::IdentValue(FullIdent::new(vec!["my".to_string(), "ident".to_string(), "value".to_string()])));
}

#[test]
fn parse_option_with_full_ident_name() {
    //not really allowed in the grammar, () are missing
    let input = min_file() + "option my_option.full.ident.name = 42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].full_ident.idents.len(), 4);
    assert_eq!(result.options[0].full_ident.idents[0], "my_option".to_string());
    assert_eq!(result.options[0].full_ident.idents[1], "full".to_string());
    assert_eq!(result.options[0].full_ident.idents[2], "ident".to_string());
    assert_eq!(result.options[0].full_ident.idents[3], "name".to_string());

    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_with_full_ident_name_in_paren() {
    //not really allowed in the grammar, () are missing
    let input = min_file() + "option (my_option.full.ident.name ) = 42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].full_ident.idents.len(), 4);
    assert_eq!(result.options[0].full_ident.idents[0], "my_option".to_string());
    assert_eq!(result.options[0].full_ident.idents[1], "full".to_string());
    assert_eq!(result.options[0].full_ident.idents[2], "ident".to_string());
    assert_eq!(result.options[0].full_ident.idents[3], "name".to_string());

    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_with_plus_prefixed_number() {
    let input = min_file() + "option foo = +42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(42.0));
}

#[test]
fn parse_option_with_minus_prefixed_number() {
    let input = min_file() + "option foo = -42;";
    let result = parse(&input).unwrap();

    assert_eq!(result.options.len(), 1);
    assert_eq!(result.options[0].constant, ConstantValue::NumberValue(-42.0));
}

// helper methods

fn min_file() -> String {
    return "syntax = \"proto3\";".to_string();
}
