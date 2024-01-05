#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IPv4 {
    pub value: u32,
    pub tolerance: Option<usize>,
}
