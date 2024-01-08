#[derive(Clone, Debug, PartialEq)]
pub struct IPv4 {
    pub value: u32,
    pub tolerance: Option<usize>,
}
