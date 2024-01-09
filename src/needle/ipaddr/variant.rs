#[derive(Clone, Debug, PartialEq)]
pub enum IPv4Variant {
    LE(Vec<u8>),
    BE(Vec<u8>),
}
