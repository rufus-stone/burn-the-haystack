use anyhow::{anyhow, Result};

use crate::needle::{
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::Timestamp,
    Interpret, Needle, Recombobulate,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TimestampVariant {
    // Unix epoch
    // Number of seconds elapsed since 1970-01-01 00:00:00Z
    UnixEpochSecs(IntegerVariant),
    UnixEpochSecsFloat(FloatVariant),
    UnixEpochMillis(IntegerVariant),
    UnixEpochMillisFloat(FloatVariant),
    UnixEpochMicros(IntegerVariant),
    UnixEpochMicrosFloat(FloatVariant),
    UnixEpochNanos(IntegerVariant),
    UnixEpochNanosFloat(FloatVariant),

    // DOS time
    DOSTime(IntegerVariant),
}

impl Recombobulate for TimestampVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            TimestampVariant::UnixEpochSecs(v) => {
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
            TimestampVariant::UnixEpochSecsFloat(v) => {
                if let Ok(Needle::Float(float)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_nanos(
                        (float.value * 1000000000.0) as i64, // Turn into nanoseconds to preserve any nanosecond precision
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch secs float"
                    ))
                }
            }
            TimestampVariant::UnixEpochMillis(v) => {
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
            TimestampVariant::UnixEpochMillisFloat(v) => {
                if let Ok(Needle::Float(float)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_nanos(
                        (float.value * 1000000.0) as i64, // Turn into nanoseconds to preserve any nanosecond precision
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch millis float"
                    ))
                }
            }
            TimestampVariant::UnixEpochMicros(v) => {
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
            TimestampVariant::UnixEpochMicrosFloat(v) => {
                if let Ok(Needle::Float(float)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_nanos(
                        (float.value * 1000.0) as i64, // Turn into nanoseconds to preserve any nanosecond precision
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch micros float"
                    ))
                }
            }
            TimestampVariant::UnixEpochNanos(v) => {
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
            TimestampVariant::UnixEpochNanosFloat(v) => {
                if let Ok(Needle::Float(float)) = v.recombobulate() {
                    Ok(Needle::Timestamp(Timestamp::from_epoch_nanos(
                        float.value as i64,
                    )?))
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Timestamp from epoch nanos float"
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
                intepretations.push(TimestampVariant::UnixEpochSecs(variant.clone()));
                intepretations.push(TimestampVariant::UnixEpochMillis(variant.clone()));
                intepretations.push(TimestampVariant::UnixEpochMicros(variant.clone()));
                intepretations.push(TimestampVariant::UnixEpochNanos(variant.clone()));

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

        // Next, interpret as floats
        if let Ok(float_variants) = FloatVariant::interpret(data) {
            for variant in &float_variants {
                intepretations.push(TimestampVariant::UnixEpochSecsFloat(variant.clone()));
                intepretations.push(TimestampVariant::UnixEpochMillisFloat(variant.clone()));
                intepretations.push(TimestampVariant::UnixEpochMicrosFloat(variant.clone()));
                intepretations.push(TimestampVariant::UnixEpochNanosFloat(variant.clone()));
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
        // println!("{:02x?}", target.discombobulate());

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
