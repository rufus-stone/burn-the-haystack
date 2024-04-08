use crate::needle::{variant::NeedleVariant, Needle};

#[derive(Debug)]
pub struct Ashes<'a> {
    pub target: &'a Needle,
    pub actual: Needle,
    pub variant: NeedleVariant,
    pub offset: usize,
}

impl<'a> Ashes<'a> {
    pub fn new(target: &'a Needle, actual: Needle, variant: NeedleVariant, offset: usize) -> Self {
        Self {
            target,
            actual,
            variant,
            offset,
        }
    }
}
