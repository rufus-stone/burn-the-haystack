use anyhow::{anyhow, Result};
use time::PrimitiveDateTime;

use crate::needle::{number::variants::IntegerVariant, Interpret, Needle, Recombobulate};

// #[derive(Clone, Debug, PartialEq)]
// pub enum TimestampVariant {
//     // Epoch seconds
//     EpochSecsLE(Vec<u8>),
//     EpochSecsBE(Vec<u8>),
//     EpochSecsVarint(Vec<u8>),

//     // Epoch millis
//     EpochMillisLE(Vec<u8>),
//     EpochMillisBE(Vec<u8>),
//     EpochMillisVarint(Vec<u8>),

//     // Epoch micros
//     EpochMicrosLE(Vec<u8>),
//     EpochMicrosBE(Vec<u8>),
//     EpochMicrosVarint(Vec<u8>),

//     // Epoch nanos
//     EpochNanosLE(Vec<u8>),
//     EpochNanosBE(Vec<u8>),
//     EpochNanosVarint(Vec<u8>),

//     // DOS time
//     DOSTimeLE(Vec<u8>),
//     DOSTimeBE(Vec<u8>),
// }

#[derive(Clone, Debug, PartialEq)]
pub enum TimestampVariant {
    // Epoch seconds
    EpochSecs(IntegerVariant),

    // Epoch millis
    EpochMillis(IntegerVariant),

    // Epoch micros
    EpochMicros(IntegerVariant),

    // Epoch nanos
    EpochNanos(IntegerVariant),

    // DOS time
    DOSTime(IntegerVariant),
}

impl TimestampVariant {
    pub fn as_epoch_secs_le(data: &[u8]) -> Result<(TimestampVariant, PrimitiveDateTime)> {
        todo!()
        //let i = u32::from_le_bytes(data[0..4].try_into()?);
    }
}

impl Recombobulate for TimestampVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            TimestampVariant::EpochSecs(v) => todo!(),
            TimestampVariant::EpochMillis(v) => todo!(),
            TimestampVariant::EpochMicros(v) => todo!(),
            TimestampVariant::EpochNanos(v) => todo!(),
            TimestampVariant::DOSTime(v) => todo!(),
        }
    }
}

// impl Recombobulate for TimestampVariant {
//     fn recombobulate(&self) -> Result<Needle> {
//         match self {
//             TimestampVariant::EpochSecs(v) => {
//                 if let Ok(needle) = v.recombobulate() {
//                     if let Needle::Integer(integer) = needle {
//                         let v = integer.value;
//                     }
//                     todo!()
//                 } else {
//                     Err(anyhow!(
//                         "Failed to recreate Needle::Timestamp from TimestampVariant::EpochSecs"
//                     ))
//                 }
//             }
//             TimestampVariant::EpochMillis(_) => todo!(),
//             TimestampVariant::EpochMicros(_) => todo!(),
//             TimestampVariant::EpochNanos(_) => todo!(),
//             TimestampVariant::DOSTime(_) => todo!(),
//         }
//     }
// }

impl Interpret for TimestampVariant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        // First, interpret as integers, as these are the basis for most of our timestamp formats
        if let Ok(integer_variants) = IntegerVariant::interpret(data) {
            for variant in &integer_variants {
                intepretations.push(TimestampVariant::EpochSecs(variant.clone()));
                intepretations.push(TimestampVariant::EpochMillis(variant.clone()));
                intepretations.push(TimestampVariant::EpochMicros(variant.clone()));
                intepretations.push(TimestampVariant::EpochNanos(variant.clone()));
            }
        }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid TimestampVariant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn timestamp_variants() {
        // DTG: 2023-12-31 23:59:58
        let data = vec![0x7eu8, 0x00, 0x92, 0x65]; // EpochSecsLE

        //let r = TimestampVariant::as_epoch_secs(&data);
        //assert!(r.is_ok());
        //assert_eq!(r.unwrap().1, 32);

        let interps = TimestampVariant::interpret(&data);

        // DTG: 2023-12-31 23:59:58
        // Timestamp(EpochSecsLE([7e, 00, 92, 65]))
        // Timestamp(EpochSecsBE([65, 92, 00, 7e]))
        // Timestamp(EpochSecsVarint([fc, 81, 90, d9, 0c]))
        // Timestamp(EpochMillisLE([30, ec, 51, c2, 8c, 01, 00, 00]))
        // Timestamp(EpochMillisBE([00, 00, 01, 8c, c2, 51, ec, 30]))
        // Timestamp(EpochMillisVarint([e0, b0, 8f, a5, 98, 63]))
        // Timestamp(EpochMicrosLE([80, 9b, 02, 10, d7, 0d, 06, 00]))
        // Timestamp(EpochMicrosBE([00, 06, 0d, d7, 10, 02, 9b, 80]))
        // Timestamp(EpochMicrosVarint([80, ee, 94, 80, e2, f5, 86, 06]))
        // Timestamp(EpochNanosLE([00, 6c, 2f, 8a, 16, 10, a6, 17]))
        // Timestamp(EpochNanosBE([17, a6, 10, 16, 8a, 2f, 6c, 00]))
        // Timestamp(EpochNanosVarint([80, b0, fb, a2, d1, 85, 88, a6, 2f]))
        // Timestamp(DOSTimeLE([7d, bf, 9f, 57]))
        // Timestamp(DOSTimeBE([57, 9f, bf, 7d]))
        // DOS: 1470087037
    }
}
