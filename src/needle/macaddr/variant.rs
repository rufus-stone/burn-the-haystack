#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MACAddrVariant {
    LE([u8; 6]),
    BE([u8; 6]),
}
