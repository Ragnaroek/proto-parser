
#[derive(Debug, PartialEq)]
pub enum Syntax {
    V3
}

#[derive(Debug)]
pub struct ProtoDef {
    pub syntax: Syntax,
    pub imports: Vec<Import>,
    pub packages: Vec<Package>
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

#[derive(Debug)]
pub struct FullIdent {
    pub idents: Vec<String>
}

#[derive(Debug)]
pub struct Package {
    pub full_ident: FullIdent
}



impl ProtoDef {
    pub fn new(syn: Syntax) -> ProtoDef {
        return ProtoDef{syntax: syn, imports: Vec::new(), packages: Vec::new()};
    }

    pub fn add_import(&mut self, imp: Import) {
        self.imports.push(imp);
    }

    pub fn add_package(&mut self, pckg: Package) {
        self.packages.push(pckg);
    }
}
