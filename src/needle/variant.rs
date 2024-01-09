use anyhow::{anyhow, Result};

use super::{
    location::variant::LocationVariant,
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::variants::TimestampVariant,
    Needle, Recombobulate,
};

/// Enum to represent all the possible byte sequences for each variant
#[derive(Clone, Debug, PartialEq)]
pub enum NeedleVariant {
    Integer(IntegerVariant),
    Float(FloatVariant),
    Timestamp(TimestampVariant),
    Location(LocationVariant),
}

impl NeedleVariant {
    pub fn byte_sequence(&self) -> &[u8] {
        match self {
            NeedleVariant::Integer(v) => v.byte_sequence(),
            NeedleVariant::Float(v) => todo!(),
            NeedleVariant::Timestamp(v) => todo!(),
            NeedleVariant::Location(v) => todo!(),
        }
    }
}

impl Recombobulate for NeedleVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            NeedleVariant::Integer(integer_variant) => integer_variant.recombobulate(),
            NeedleVariant::Float(float_variant) => float_variant.recombobulate(),
            NeedleVariant::Timestamp(_) => todo!(),
            NeedleVariant::Location(_) => todo!(),
        }
    }
}
