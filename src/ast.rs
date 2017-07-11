
#[derive(Debug)]
pub enum Syntax {
    V3
}

#[derive(Debug)]
pub struct ProtoDef {
    pub syntax: Syntax,
}
