
#[derive(Debug, PartialEq)]
pub enum Syntax {
    V3
}

#[derive(Debug)]
pub struct ProtoDef {
    pub syntax: Syntax,
    pub imports: Vec<Import>
}

#[derive(Debug, PartialEq)]
pub enum ImportType {
    Default,
    Weak,
    Public
}

#[derive(Debug)]
pub struct Import {
    pub import_type: ImportType,
    pub name: String
}

impl ProtoDef {
    pub fn new(syn: Syntax) -> ProtoDef {
        return ProtoDef{syntax: syn, imports: Vec::new()};
    }

    pub fn add_import(&mut self, imp: Import) {
        self.imports.push(imp);
    }
}
