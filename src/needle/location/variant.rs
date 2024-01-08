#[derive(Clone, Debug, PartialEq)]
pub enum LocationVariant {
    // Decimal degrees
    DecimalDegreesF64LE(Vec<u8>),
    DecimalDegreesF64BE(Vec<u8>),
    DecimalDegreesF32LE(Vec<u8>),
    DecimalDegreesF32BE(Vec<u8>),
}
