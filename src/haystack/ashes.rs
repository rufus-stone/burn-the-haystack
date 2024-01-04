use crate::needle::Needle;

#[derive(Debug)]
pub struct Ashes<'a> {
    pub needle: &'a Needle,
    pub pos: usize,
    pub name: String,
    pub byte_seq: Vec<u8>,
}

impl<'a> Ashes<'a> {
    pub fn new(needle: &'a Needle, pos: usize, name: String, byte_seq: Vec<u8>) -> Self {
        Self {
            needle,
            pos,
            name,
            byte_seq,
        }
    }
}
