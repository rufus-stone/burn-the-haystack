#[derive(Clone, Debug, PartialEq)]
pub enum IntegerVariant {
    // u8
    U8(Vec<u8>),
    U8Varint(Vec<u8>),

    // i8
    I8(Vec<u8>),
    I8Varint(Vec<u8>),

    // u16
    U16LE(Vec<u8>),
    U16BE(Vec<u8>),
    U16Varint(Vec<u8>),

    // i16
    I16LE(Vec<u8>),
    I16BE(Vec<u8>),
    I16Varint(Vec<u8>),

    // u32
    U32LE(Vec<u8>),
    U32BE(Vec<u8>),
    U32Varint(Vec<u8>),

    // i32
    I32LE(Vec<u8>),
    I32BE(Vec<u8>),
    I32Varint(Vec<u8>),

    // u64
    U64LE(Vec<u8>),
    U64BE(Vec<u8>),
    U64Varint(Vec<u8>),

    // i64
    I64LE(Vec<u8>),
    I64BE(Vec<u8>),
    I64Varint(Vec<u8>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum FloatVariant {
    // f32
    F32LE(Vec<u8>),
    F32BE(Vec<u8>),

    // f64
    F64LE(Vec<u8>),
    F64BE(Vec<u8>),
}
