extern crate protoparser;

use protoparser::ast::*;

#[test]
fn should_insert_into_full_ident() {
    let mut ident = FullIdent::new(vec!["foo".to_string(), "bar".to_string()]);
    ident.insert(0, "baz".to_string());
    assert_eq!(ident.idents[0], "baz".to_string());
}

#[test]
fn should_add_import() {
    let mut def = ProtoDef::new(Syntax::V3);
    assert_eq!(def.imports.len(), 0);
    def.add_import(Import{import_type: ImportType::Default, name: "test".to_string()});
    assert_eq!(def.imports.len(), 1);
    assert_eq!(def.imports[0].name, "test".to_string());

    def.add_import(Import{import_type: ImportType::Weak, name: "test2".to_string()});
    assert_eq!(def.imports.len(), 2);
    assert_eq!(def.imports[0].name, "test".to_string());
    assert_eq!(def.imports[1].name, "test2".to_string());
}

#[test]
fn should_add_package() {
    let mut def = ProtoDef::new(Syntax::V3);
    assert_eq!(def.packages.len(), 0);
    def.add_package(Package{full_ident: FullIdent::new(vec!["foo".to_string()])});
    assert_eq!(def.packages.len(), 1);
    def.add_package(Package{full_ident: FullIdent::new(vec!["foo".to_string(), "bar".to_string()])});
    assert_eq!(def.packages.len(), 2);

    assert_eq!(def.packages[1].full_ident.idents.len(), 2);
}

#[test]
fn should_add_option() {
    let mut def = ProtoDef::new(Syntax::V3);
    assert_eq!(def.options.len(), 0);
    def.add_option(ProtoOption{
        full_ident: FullIdent::new(vec!["foo".to_string()]),
        constant: ConstantValue::BoolValue(true)
    });
    assert_eq!(def.options.len(), 1);
    def.add_option(ProtoOption{
        full_ident: FullIdent::new(vec!["bar".to_string()]),
        constant: ConstantValue::NumberValue(666.0)
    });
    assert_eq!(def.options.len(), 2);

    assert_eq!(def.options[1].constant, ConstantValue::NumberValue(666.0));
}
