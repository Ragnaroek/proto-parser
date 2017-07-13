
#[derive(Debug)]
pub enum Syntax {
    V3
}

#[derive(Debug)]
pub struct ProtoDef {
    pub syntax: Syntax,
    pub imports: Vec<Import>
}

#[derive(Debug)]
pub struct Import {
}

impl ProtoDef {
    pub fn new(syn: Syntax) -> ProtoDef {
        return ProtoDef{syntax: syn, imports: Vec::new()};
    }

    pub fn add_import(&mut self, imp: Import) {
        self.imports.push(imp);
    }
}
