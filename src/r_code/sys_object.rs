use super::Atom;

#[derive(Default)]
pub struct SysObject {
    pub code_: Vec<Atom>,
    pub references_: Vec<u16>,
}
