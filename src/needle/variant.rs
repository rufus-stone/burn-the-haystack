use super::{
    location::variant::LocationVariant,
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::variants::TimestampVariant,
};

/// Enum to represent all the possible byte sequences for each variant
#[derive(Clone, Debug, PartialEq)]
pub enum NeedleVariant {
    Integer(IntegerVariant),
    Float(FloatVariant),
    Timestamp(TimestampVariant),
    Location(LocationVariant),
}
