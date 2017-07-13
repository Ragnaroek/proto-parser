extern crate protoparse;

use protoparse::ast::{ProtoDef, Syntax, Import};

#[test]
fn should_add_import() {
    let mut def = ProtoDef::new(Syntax::V3);
    assert_eq!(def.imports.len(), 0);
    def.add_import(Import{});
    assert_eq!(def.imports.len(), 1);
}
