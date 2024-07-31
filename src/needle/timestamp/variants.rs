use anyhow::{anyhow, Result};

use crate::needle::{
    number::variants::IntegerVariant, timestamp::Timestamp, Interpret, Needle, Recombobulate,
};

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

impl Recombobulate for TimestampVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            TimestampVariant::EpochSecs(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_secs(
                        integer.value,
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch secs"
                    ))
                }
            }
            TimestampVariant::EpochMillis(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_millis(
                        integer.value,
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch millis"
                    ))
                }
            }
            TimestampVariant::EpochMicros(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_micros(
                        integer.value,
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch micros"
                    ))
                }
            }
            TimestampVariant::EpochNanos(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_nanos(
                        integer.value,
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch nanos"
                    ))
                }
            }
            TimestampVariant::DOSTime(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    if (u32::MIN as i64..=u32::MAX as i64).contains(&integer.value) {
                        Ok(Needle::Timestamp(Timestamp::from_dos_time(
                            integer.value as u32,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Timestamp from DOS time"
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from DOS time"
                    ))
                }
            }
        }
    }
}

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

                // Test for valid DOS Times for u32 variants
                if match variant {
                    IntegerVariant::U32LE((_, i)) => Timestamp::from_dos_time(*i).is_ok(),
                    IntegerVariant::U32BE((_, i)) => Timestamp::from_dos_time(*i).is_ok(),
                    IntegerVariant::U32Varint((_, i)) => Timestamp::from_dos_time(*i).is_ok(),
                    _ => false,
                } {
                    intepretations.push(TimestampVariant::DOSTime(variant.clone()))
                }
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

    use crate::needle::Matches;

    use super::*;

    #[test]
    fn timestamp_variants() {
        // DTG: 2023-12-31 23:59:58
        let data = vec![0x7eu8, 0x00, 0x92, 0x65]; // EpochSecsLE

        let target = Needle::new_timestamp("2023-12-31 23:59:58").unwrap();
        //println!("{:02x?}", target.discombobulate());

        let interps = TimestampVariant::interpret(&data);
        assert!(interps.is_ok());

        if let Ok(interps) = TimestampVariant::interpret(&data) {
            for timestamp_variant in &interps {
                // println!("{:?}", &timestamp_variant);

                if let Ok(timestamp) = timestamp_variant.recombobulate() {
                    // println!("{:?}", &timestamp);

                    if timestamp.matches(&target) {
                        println!("[ Target needle found ]");
                        println!("Target  : {:?}", target);
                        println!("Actual  : {:?}", timestamp);
                        println!("Variant : {:02x?}", timestamp_variant);
                    }
                }
            }
        }

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
