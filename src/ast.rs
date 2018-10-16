
#[derive(Debug, PartialEq)]
pub enum Syntax {
    V3
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

#[derive(Debug, PartialEq)]
pub struct FullIdent {
    pub idents: Vec<String>
}

impl FullIdent {
    pub fn new(v: Vec<String>) -> FullIdent {
        return FullIdent{idents: v};
    }

    pub fn insert(&mut self, ix: usize, n: String) {
        self.idents.insert(ix, n);
    }
}

#[derive(Debug)]
pub struct Package {
    pub full_ident: FullIdent
}

#[derive(Debug)]
pub struct ProtoOption {
    pub full_ident: FullIdent,
    pub constant: ConstantValue
}

#[derive(Debug, PartialEq)]
pub enum ConstantValue {
    IdentValue(FullIdent),
    NumberValue(f32),
    StringValue(String),
    BoolValue(bool)
}

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub rpcs: Vec<Rpc>
}

#[derive(Debug)]
pub struct Rpc {
    pub name: String,
    pub request_type: FullIdent,
    pub response_type: FullIdent
}

#[derive(Debug)]
pub struct ProtoDef {
    pub syntax: Syntax,
    pub imports: Vec<Import>,
    pub packages: Vec<Package>,
    pub options: Vec<ProtoOption>,
    pub services: Vec<Service>,
}

impl ProtoDef {
    pub fn new(syn: Syntax) -> ProtoDef {
        return ProtoDef{
            syntax: syn,
            imports: Vec::new(),
            packages: Vec::new(),
            options: Vec::new(),
            services: Vec::new(),
        };
    }

    pub fn add_import(&mut self, imp: Import) {
        self.imports.push(imp);
    }

    pub fn add_package(&mut self, pckg: Package) {
        self.packages.push(pckg);
    }

    pub fn add_option(&mut self, opt: ProtoOption) {
        self.options.push(opt);
    }

    pub fn add_service(&mut self, service: Service) {
        self.services.push(service);
    }
}
