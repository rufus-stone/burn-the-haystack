use anyhow::{anyhow, Result};
use integer_encoding::VarInt;

use crate::needle::{Interpret, Needle, Recombobulate};

#[derive(Clone, Debug, PartialEq)]
pub enum IntegerVariant {
    // u8
    U8((Vec<u8>, u8)),
    U8Varint((Vec<u8>, u8)),

    // i8
    I8((Vec<u8>, i8)),
    I8Varint((Vec<u8>, i8)),

    // u16
    U16LE((Vec<u8>, u16)),
    U16BE((Vec<u8>, u16)),
    U16Varint((Vec<u8>, u16)),

    // i16
    I16LE((Vec<u8>, i16)),
    I16BE((Vec<u8>, i16)),
    I16Varint((Vec<u8>, i16)),

    // u32
    U32LE((Vec<u8>, u32)),
    U32BE((Vec<u8>, u32)),
    U32Varint((Vec<u8>, u32)),

    // i32
    I32LE((Vec<u8>, i32)),
    I32BE((Vec<u8>, i32)),
    I32Varint((Vec<u8>, i32)),

    // u64
    U64LE((Vec<u8>, u64)),
    U64BE((Vec<u8>, u64)),
    U64Varint((Vec<u8>, u64)),

    // i64
    I64LE((Vec<u8>, i64)),
    I64BE((Vec<u8>, i64)),
    I64Varint((Vec<u8>, i64)),
}

impl IntegerVariant {
    /// 8 bit values
    ///
    pub fn as_u8(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u8)> {
        // let i = u8::from_ne_bytes(data[0..1].try_into()?);
        // Ok((IntegerVariant::U8(data[0..1].to_owned()), i))

        if !data.is_empty() {
            let i = u8::from_ne_bytes(data[0..1].try_into()?);
            //Ok((IntegerVariant::U8(data[0..1].to_owned()), i))
            Ok(IntegerVariant::U8((data[0..1].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U8!"
            ))
        }
    }

    pub fn as_u8_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u8)> {
        //let (i, len) = u8::decode_var(data).unwrap();
        // Ok((IntegerVariant::U8Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = u8::decode_var(data) {
            //Ok((IntegerVariant::U8Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::U8Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::U8Varint from bytes!"
            ))
        }
    }

    pub fn as_i8(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i8)> {
        // let i = i8::from_ne_bytes(data[0..1].try_into()?);
        // Ok((IntegerVariant::I8(data[0..1].to_owned()), i))

        if !data.is_empty() {
            let i = i8::from_ne_bytes(data[0..1].try_into()?);
            //Ok((IntegerVariant::I8(data[0..1].to_owned()), i))
            Ok(IntegerVariant::I8((data[0..1].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I8!"
            ))
        }
    }

    pub fn as_i8_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i8)> {
        // let (i, len) = i8::decode_var(data).unwrap();
        // Ok((IntegerVariant::I8Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = i8::decode_var(data) {
            //Ok((IntegerVariant::I8Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::I8Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::I8Varint from bytes!"
            ))
        }
    }

    /// 16 bit values
    ///
    pub fn as_u16_le(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u16)> {
        // let i = u16::from_le_bytes(data[0..2].try_into()?);
        // Ok((IntegerVariant::U16LE(data[0..2].to_owned()), i))

        if data.len() >= 2 {
            let i = u16::from_le_bytes(data[0..2].try_into()?);
            //Ok((IntegerVariant::U16LE(data[0..2].to_owned()), i))
            Ok(IntegerVariant::U16LE((data[0..2].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U16LE!"
            ))
        }
    }

    pub fn as_u16_be(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u16)> {
        // let i = u16::from_be_bytes(data[0..2].try_into()?);
        // Ok((IntegerVariant::U16BE(data[0..2].to_owned()), i))

        if data.len() >= 2 {
            let i = u16::from_be_bytes(data[0..2].try_into()?);
            //Ok((IntegerVariant::U16BE(data[0..2].to_owned()), i))
            Ok(IntegerVariant::U16BE((data[0..2].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U16BE!"
            ))
        }
    }

    pub fn as_u16_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u16)> {
        // let (i, len) = u16::decode_var(data).unwrap();
        // Ok((IntegerVariant::U16Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = u16::decode_var(data) {
            //Ok((IntegerVariant::U16Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::U16Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::U16Varint from bytes!"
            ))
        }
    }

    pub fn as_i16_le(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i16)> {
        // let i = i16::from_le_bytes(data[0..2].try_into()?);
        // Ok((IntegerVariant::I16LE(data[0..2].to_owned()), i))

        if data.len() >= 2 {
            let i = i16::from_le_bytes(data[0..2].try_into()?);
            //Ok((IntegerVariant::I16LE(data[0..2].to_owned()), i))
            Ok(IntegerVariant::I16LE((data[0..2].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I16LE!"
            ))
        }
    }

    pub fn as_i16_be(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i16)> {
        // let i = i16::from_be_bytes(data[0..2].try_into()?);
        // Ok((IntegerVariant::I16BE(data[0..2].to_owned()), i))

        if data.len() >= 2 {
            let i = i16::from_be_bytes(data[0..2].try_into()?);
            //Ok((IntegerVariant::I16BE(data[0..2].to_owned()), i))
            Ok(IntegerVariant::I16BE((data[0..2].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I16BE!"
            ))
        }
    }

    pub fn as_i16_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i16)> {
        // let (i, len) = i16::decode_var(data).unwrap();
        // Ok((IntegerVariant::I16Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = i16::decode_var(data) {
            //Ok((IntegerVariant::I16Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::I16Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::I16Varint from bytes!"
            ))
        }
    }

    /// 32 bit values
    ///
    pub fn as_u32_le(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u32)> {
        // let i = u32::from_le_bytes(data[0..4].try_into()?);
        // Ok((IntegerVariant::U32LE(data[0..4].to_owned()), i))

        if data.len() >= 4 {
            let i = u32::from_le_bytes(data[0..4].try_into()?);
            //Ok((IntegerVariant::U32LE(data[0..4].to_owned()), i))
            Ok(IntegerVariant::U32LE((data[0..4].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U32LE!"
            ))
        }
    }

    pub fn as_u32_be(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u32)> {
        // let i = u32::from_be_bytes(data[0..4].try_into()?);
        // Ok((IntegerVariant::U32BE(data[0..4].to_owned()), i))

        if data.len() >= 4 {
            let i = u32::from_be_bytes(data[0..4].try_into()?);
            //Ok((IntegerVariant::U32BE(data[0..4].to_owned()), i))
            Ok(IntegerVariant::U32BE((data[0..4].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U32BE!"
            ))
        }
    }

    pub fn as_u32_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u32)> {
        // let (i, len) = u32::decode_var(data).unwrap();
        // Ok((IntegerVariant::U32Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = u32::decode_var(data) {
            //Ok((IntegerVariant::U32Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::U32Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::U32Varint from bytes!"
            ))
        }
    }

    pub fn as_i32_le(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i32)> {
        // let i = i32::from_le_bytes(data[0..4].try_into()?);
        // Ok((IntegerVariant::I32LE(data[0..4].to_owned()), i))

        if data.len() >= 4 {
            let i = i32::from_le_bytes(data[0..4].try_into()?);
            //Ok((IntegerVariant::I32LE(data[0..4].to_owned()), i))
            Ok(IntegerVariant::I32LE((data[0..4].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I32LE!"
            ))
        }
    }

    pub fn as_i32_be(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i32)> {
        // let i = i32::from_be_bytes(data[0..4].try_into()?);
        // Ok((IntegerVariant::I32BE(data[0..4].to_owned()), i))

        if data.len() >= 4 {
            let i = i32::from_be_bytes(data[0..4].try_into()?);
            //Ok((IntegerVariant::I32BE(data[0..4].to_owned()), i))
            Ok(IntegerVariant::I32BE((data[0..4].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I32BE!"
            ))
        }
    }

    pub fn as_i32_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i32)> {
        // let (i, len) = i32::decode_var(data).unwrap();
        // Ok((IntegerVariant::I32Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = i32::decode_var(data) {
            //Ok((IntegerVariant::I32Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::I32Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::I32Varint from bytes!"
            ))
        }
    }

    /// 64 bit values
    ///
    pub fn as_u64_le(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u64)> {
        // let i = u64::from_le_bytes(data[0..8].try_into()?);
        // Ok((IntegerVariant::U64LE(data[0..8].to_owned()), i))

        if data.len() >= 8 {
            let i = u64::from_le_bytes(data[0..8].try_into()?);
            //Ok((IntegerVariant::U64LE(data[0..8].to_owned()), i))
            Ok(IntegerVariant::U64LE((data[0..8].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U64LE!"
            ))
        }
    }

    pub fn as_u64_be(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u64)> {
        // let i = u64::from_be_bytes(data[0..8].try_into()?);
        // Ok((IntegerVariant::U64BE(data[0..8].to_owned()), i))

        if data.len() >= 8 {
            let i = u64::from_be_bytes(data[0..8].try_into()?);
            //Ok((IntegerVariant::U64BE(data[0..8].to_owned()), i))
            Ok(IntegerVariant::U64BE((data[0..8].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::U64BE!"
            ))
        }
    }

    pub fn as_u64_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, u64)> {
        // let (i, len) = u64::decode_var(data).unwrap();
        // Ok((IntegerVariant::U64Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = u64::decode_var(data) {
            //Ok((IntegerVariant::U64Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::U64Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::U64Varint from bytes!"
            ))
        }
    }

    pub fn as_i64_le(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i64)> {
        // let i = i64::from_le_bytes(data[0..8].try_into()?);
        // Ok((IntegerVariant::I64LE(data[0..8].to_owned()), i))

        if data.len() >= 8 {
            let i = i64::from_le_bytes(data[0..8].try_into()?);
            //Ok((IntegerVariant::I64LE(data[0..8].to_owned()), i))
            Ok(IntegerVariant::I64LE((data[0..8].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I64LE!"
            ))
        }
    }

    pub fn as_i64_be(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i64)> {
        // let i = i64::from_be_bytes(data[0..8].try_into()?);
        // Ok((IntegerVariant::I64BE(data[0..8].to_owned()), i))

        if data.len() >= 8 {
            let i = i64::from_be_bytes(data[0..8].try_into()?);
            //Ok((IntegerVariant::I64BE(data[0..8].to_owned()), i))
            Ok(IntegerVariant::I64BE((data[0..8].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a IntegerVariant::I64BE!"
            ))
        }
    }

    pub fn as_i64_varint(data: &[u8]) -> Result<IntegerVariant> {
        //Result<(IntegerVariant, i64)> {
        // let (i, len) = i64::decode_var(data).unwrap();
        // Ok((IntegerVariant::I64Varint(data[0..len].to_owned()), i))

        if let Some((i, len)) = i64::decode_var(data) {
            //Ok((IntegerVariant::I64Varint(data[0..len].to_owned()), i))
            Ok(IntegerVariant::I64Varint((data[0..len].to_owned(), i)))
        } else {
            Err(anyhow!(
                "Failed to build IntegerVariant::I64Varint from bytes!"
            ))
        }
    }

    pub fn byte_sequence(&self) -> &[u8] {
        match self {
            IntegerVariant::U8(v) => &v.0,
            IntegerVariant::U8Varint(v) => &v.0,
            IntegerVariant::I8(v) => &v.0,
            IntegerVariant::I8Varint(v) => &v.0,
            IntegerVariant::U16LE(v) => &v.0,
            IntegerVariant::U16BE(v) => &v.0,
            IntegerVariant::U16Varint(v) => &v.0,
            IntegerVariant::I16LE(v) => &v.0,
            IntegerVariant::I16BE(v) => &v.0,
            IntegerVariant::I16Varint(v) => &v.0,
            IntegerVariant::U32LE(v) => &v.0,
            IntegerVariant::U32BE(v) => &v.0,
            IntegerVariant::U32Varint(v) => &v.0,
            IntegerVariant::I32LE(v) => &v.0,
            IntegerVariant::I32BE(v) => &v.0,
            IntegerVariant::I32Varint(v) => &v.0,
            IntegerVariant::U64LE(v) => &v.0,
            IntegerVariant::U64BE(v) => &v.0,
            IntegerVariant::U64Varint(v) => &v.0,
            IntegerVariant::I64LE(v) => &v.0,
            IntegerVariant::I64BE(v) => &v.0,
            IntegerVariant::I64Varint(v) => &v.0,
        }
    }
}

impl Recombobulate for IntegerVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            IntegerVariant::U8(v) => {
                if let Ok(variant) = IntegerVariant::as_u8(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U8"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U8"
                    ))
                }
            }
            IntegerVariant::U8Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_u8_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U8Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U8Varint"
                    ))
                }
            }
            IntegerVariant::I8(v) => {
                if let Ok(variant) = IntegerVariant::as_i8(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I8"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I8"
                    ))
                }
            }
            IntegerVariant::I8Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_i8_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I8Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I8Varint"
                    ))
                }
            }
            IntegerVariant::U16LE(v) => {
                if let Ok(variant) = IntegerVariant::as_u16_le(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U16LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U16LE"
                    ))
                }
            }
            IntegerVariant::U16BE(v) => {
                if let Ok(variant) = IntegerVariant::as_u16_be(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U16BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U16BE"
                    ))
                }
            }
            IntegerVariant::U16Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_u16_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U16Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U16Varint"
                    ))
                }
            }
            IntegerVariant::I16LE(v) => {
                if let Ok(variant) = IntegerVariant::as_i16_le(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I16LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I16LE"
                    ))
                }
            }
            IntegerVariant::I16BE(v) => {
                if let Ok(variant) = IntegerVariant::as_i16_be(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I16BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I16BE"
                    ))
                }
            }
            IntegerVariant::I16Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_i16_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I16Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I16Varint"
                    ))
                }
            }
            IntegerVariant::U32LE(v) => {
                if let Ok(variant) = IntegerVariant::as_u32_le(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U32LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U32LE"
                    ))
                }
            }
            IntegerVariant::U32BE(v) => {
                if let Ok(variant) = IntegerVariant::as_u32_be(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U32BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U32BE"
                    ))
                }
            }
            IntegerVariant::U32Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_u32_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U32Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U32Varint"
                    ))
                }
            }
            IntegerVariant::I32LE(v) => {
                if let Ok(variant) = IntegerVariant::as_i32_le(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I32LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I32LE"
                    ))
                }
            }
            IntegerVariant::I32BE(v) => {
                if let Ok(variant) = IntegerVariant::as_i32_be(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I32BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I32BE"
                    ))
                }
            }
            IntegerVariant::I32Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_i32_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I32Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I32Varint"
                    ))
                }
            }
            IntegerVariant::U64LE(v) => {
                if let Ok(variant) = IntegerVariant::as_u64_le(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U64LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U64LE"
                    ))
                }
            }
            IntegerVariant::U64BE(v) => {
                if let Ok(variant) = IntegerVariant::as_u64_be(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U64BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U64BE"
                    ))
                }
            }
            IntegerVariant::U64Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_u64_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1 as i64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::U64Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::U64Varint"
                    ))
                }
            }
            IntegerVariant::I64LE(v) => {
                if let Ok(variant) = IntegerVariant::as_i64_le(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I64LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I64LE"
                    ))
                }
            }
            IntegerVariant::I64BE(v) => {
                if let Ok(variant) = IntegerVariant::as_i64_be(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I64BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I64BE"
                    ))
                }
            }
            IntegerVariant::I64Varint(v) => {
                if let Ok(variant) = IntegerVariant::as_i64_varint(self.byte_sequence()) {
                    if let Some(needle) = Needle::new_integer(v.1) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Integer from IntegerVariant::I64Varint"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Integer from IntegerVariant::I64Varint"
                    ))
                }
            }
        }
    }
}

impl Interpret for IntegerVariant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        // 8 bit values
        //
        if let Ok(v) = IntegerVariant::as_u8(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u8_varint(data) {
            intepretations.push(v);
        }

        // 16 bit values
        //
        if let Ok(v) = IntegerVariant::as_u16_le(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u16_be(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u16_varint(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i16_le(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i16_be(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i16_varint(data) {
            intepretations.push(v);
        }

        // 32 bit values
        //
        if let Ok(v) = IntegerVariant::as_u32_le(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u32_be(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u32_varint(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i32_le(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i32_be(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i32_varint(data) {
            intepretations.push(v);
        }

        // 64 bit values
        //
        if let Ok(v) = IntegerVariant::as_u64_le(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u64_be(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_u64_varint(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i64_le(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i64_be(data) {
            intepretations.push(v);
        }

        if let Ok(v) = IntegerVariant::as_i64_varint(data) {
            intepretations.push(v);
        }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid IntegerVariant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
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

impl FloatVariant {
    /// 32 bit values
    ///
    pub fn as_f32_le(data: &[u8]) -> Result<(FloatVariant, f32)> {
        // let f = f32::from_le_bytes(data[0..4].try_into()?);
        // Ok((FloatVariant::F32LE(data[0..4].to_owned()), f))

        if data.len() >= 4 {
            let f = f32::from_le_bytes(data[0..4].try_into()?);
            Ok((FloatVariant::F32LE(data[0..4].to_owned()), f))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a FloatVariant::F32LE!"
            ))
        }
    }

    pub fn as_f32_be(data: &[u8]) -> Result<(FloatVariant, f32)> {
        // let f = f32::from_be_bytes(data[0..4].try_into()?);
        // Ok((FloatVariant::F32BE(data[0..4].to_owned()), f))

        if data.len() >= 4 {
            let f = f32::from_be_bytes(data[0..4].try_into()?);
            Ok((FloatVariant::F32BE(data[0..4].to_owned()), f))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a FloatVariant::F32BE!"
            ))
        }
    }

    /// 64 bit values
    ///
    pub fn as_f64_le(data: &[u8]) -> Result<(FloatVariant, f64)> {
        // let f = f64::from_le_bytes(data[0..8].try_into()?);
        // Ok((FloatVariant::F64LE(data[0..8].to_owned()), f))

        if data.len() >= 8 {
            let f = f64::from_le_bytes(data[0..8].try_into()?);
            Ok((FloatVariant::F64LE(data[0..8].to_owned()), f))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a FloatVariant::F64LE!"
            ))
        }
    }

    pub fn as_f64_be(data: &[u8]) -> Result<(FloatVariant, f64)> {
        // let f = f64::from_be_bytes(data[0..8].try_into()?);
        // Ok((FloatVariant::F64BE(data[0..8].to_owned()), f))

        if data.len() >= 8 {
            let f = f64::from_be_bytes(data[0..8].try_into()?);
            Ok((FloatVariant::F64BE(data[0..8].to_owned()), f))
        } else {
            Err(anyhow!(
                "Not enough data for this to be a FloatVariant::F64BE!"
            ))
        }
    }

    pub fn byte_sequence(&self) -> &[u8] {
        match self {
            FloatVariant::F32LE(v) => v,
            FloatVariant::F32BE(v) => v,
            FloatVariant::F64LE(v) => v,
            FloatVariant::F64BE(v) => v,
        }
    }
}

impl Recombobulate for FloatVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            FloatVariant::F32LE(v) => {
                if let Ok((_, f)) = FloatVariant::as_f32_le(v) {
                    if let Some(needle) = Needle::new_float(f as f64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Float from FloatVariant::F32LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Float from FloatVariant::F32LE"
                    ))
                }
            }
            FloatVariant::F32BE(v) => {
                if let Ok((_, f)) = FloatVariant::as_f32_be(v) {
                    if let Some(needle) = Needle::new_float(f as f64) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Float from FloatVariant::F32BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Float from FloatVariant::F32BE"
                    ))
                }
            }
            FloatVariant::F64LE(v) => {
                if let Ok((_, f)) = FloatVariant::as_f64_le(v) {
                    if let Some(needle) = Needle::new_float(f) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Float from FloatVariant::F64LE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Float from FloatVariant::F64LE"
                    ))
                }
            }
            FloatVariant::F64BE(v) => {
                if let Ok((_, f)) = FloatVariant::as_f64_be(v) {
                    if let Some(needle) = Needle::new_float(f) {
                        Ok(needle)
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Float from FloatVariant::F64BE"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Float from FloatVariant::F64BE"
                    ))
                }
            }
        }
    }
}

impl Interpret for FloatVariant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        // 32 bit values
        //
        if let Ok((v, _)) = FloatVariant::as_f32_le(data) {
            intepretations.push(v);
        }

        if let Ok((v, _)) = FloatVariant::as_f32_be(data) {
            intepretations.push(v);
        }

        // 64 bit values
        //
        if let Ok((v, _)) = FloatVariant::as_f64_le(data) {
            intepretations.push(v);
        }

        if let Ok((v, _)) = FloatVariant::as_f64_be(data) {
            intepretations.push(v);
        }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid FloatVariant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::f64::consts::PI;

    use crate::needle::{variant::NeedleVariant, Discombobulate, Matches};

    use super::*;

    #[test]
    fn integer_variants() {
        let data = vec![0xffu8]; // Not a valid u8 varint
        let r = IntegerVariant::as_u8_varint(&data);
        assert!(r.is_err());

        let data = vec![0x20u8]; // A valid u8 varint
        let r = IntegerVariant::as_u8_varint(&data);
        assert!(r.is_ok());

        if let IntegerVariant::U8(v) = r.unwrap() {
            assert_eq!(v.1, 32);
        }
    }

    // TODO: add comprehensive tests for valid and invalid variants for both Integer and Float

    #[test]
    fn integer_recombobulation() {
        // 240
        let needle = Needle::new_integer(240).unwrap();
        let needle_variants = needle.discombobulate();

        for needle_variant in &needle_variants {
            if let NeedleVariant::Integer(integer_variant) = needle_variant {
                let recom = integer_variant.recombobulate().unwrap();

                assert_eq!(recom, needle);
            }
        }

        // -20
        let needle = Needle::new_integer(-20).unwrap();
        let needle_variants = needle.discombobulate();

        //println!("{:?}", &needle_variants);

        for needle_variant in &needle_variants {
            if let NeedleVariant::Integer(integer_variant) = needle_variant {
                let recom = integer_variant.recombobulate().unwrap();

                assert_eq!(recom, needle);
            }
        }

        // i64::MAX
        let needle = Needle::new_integer(i64::MAX).unwrap();
        let needle_variants = needle.discombobulate();

        for needle_variant in &needle_variants {
            if let NeedleVariant::Integer(integer_variant) = needle_variant {
                let recom = integer_variant.recombobulate().unwrap();

                assert_eq!(recom, needle);
            }
        }

        // 0
        let needle = Needle::new_integer(0).unwrap();
        let needle_variants = needle.discombobulate();

        for needle_variant in &needle_variants {
            if let NeedleVariant::Integer(integer_variant) = needle_variant {
                let recom = integer_variant.recombobulate().unwrap();

                assert_eq!(recom, needle);
            }
        }

        // i64::MIN
        let needle = Needle::new_integer(i64::MIN).unwrap();
        let needle_variants = needle.discombobulate();

        for needle_variant in &needle_variants {
            if let NeedleVariant::Integer(integer_variant) = needle_variant {
                let recom = integer_variant.recombobulate().unwrap();

                assert_eq!(recom, needle);
            }
        }
    }

    #[test]
    fn integer_interpretaion() {
        let data = vec![236u8, 255, 255, 255];

        let target = Needle::new_integer(-20).unwrap();

        if let Ok(interps) = IntegerVariant::interpret(&data) {
            for integer_variant in &interps {
                println!("{:?}", &integer_variant);

                if let Ok(integer) = integer_variant.recombobulate() {
                    println!("{:?}", &integer);

                    if integer.matches(&target) {
                        println!("It's a match!");
                    }
                }
            }
        }
    }

    #[test]
    fn float_recombobulation() {
        // PI
        let needle = Needle::new_float(PI).unwrap();
        let needle_variants = needle.discombobulate();

        for needle_variant in &needle_variants {
            if let NeedleVariant::Float(float_variant) = needle_variant {
                let recom = float_variant.recombobulate().unwrap();
                // TODO: Need to account for precision differences between f32 and f64

                println!("{:?} -> {:?} -> {:?}", &needle, &float_variant, &recom);
                assert_eq!(recom, needle);
            }
        }
    }

    #[test]
    fn float_interpretaion() {
        // PI
        let data = vec![64u8, 9, 33, 251, 84, 68, 45, 24];

        if let Ok(interps) = FloatVariant::interpret(&data) {
            println!("{:?}", interps);
        }
    }

    #[test]
    fn u8_test() {
        let n: u8 = 240;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        // for variant in variants {
        //     if let NeedleVariant::Integer(integer_variant) = variant {
        //         match &integer_variant {
        //             IntegerVariant::U8(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_u8(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::U8Varint(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_u8_varint(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::I8(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_i8(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::I8Varint(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_i8_varint(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::U16LE(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_u16_le(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::U16BE(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_u16_be(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::U16Varint(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_u16_varint(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::I16LE(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_i16_le(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::I16BE(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_i16_be(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::I16Varint(v) => {
        //                 if let Ok((putative, value)) = IntegerVariant::as_i16_varint(v) {
        //                     println!("{:?} ({}) -- {:?}", putative, value, integer_variant);
        //                     assert_eq!(putative, integer_variant);
        //                 }
        //             }
        //             IntegerVariant::U32LE(v) => todo!(),
        //             IntegerVariant::U32BE(v) => todo!(),
        //             IntegerVariant::U32Varint(v) => todo!(),
        //             IntegerVariant::I32LE(v) => todo!(),
        //             IntegerVariant::I32BE(v) => todo!(),
        //             IntegerVariant::I32Varint(v) => todo!(),
        //             IntegerVariant::U64LE(v) => todo!(),
        //             IntegerVariant::U64BE(v) => todo!(),
        //             IntegerVariant::U64Varint(v) => todo!(),
        //             IntegerVariant::I64LE(v) => todo!(),
        //             IntegerVariant::I64BE(v) => todo!(),
        //             IntegerVariant::I64Varint(v) => todo!(),
        //         }
        //     }
        // }

        assert_eq!(1, 1);
    }
}
