use crate::needle::{variant::NeedleVariant, Needle};

#[derive(Debug)]
pub struct Ashes<'a> {
    pub needle: &'a Needle,
    pub needle_variant: NeedleVariant,
    pub pos: usize,
}

impl<'a> Ashes<'a> {
    pub fn new(needle: &'a Needle, needle_variant: NeedleVariant, pos: usize) -> Self {
        Self {
            needle,
            needle_variant,
            pos,
        }
    }
}
