#[derive(Clone, Debug, PartialEq)]
pub enum MACAddrVariant {
    LE(Vec<u8>),
    BE(Vec<u8>),
}
