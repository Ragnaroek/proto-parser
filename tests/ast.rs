extern crate protoparse;

use protoparse::ast::{ProtoDef, Syntax, Import, ImportType};

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
