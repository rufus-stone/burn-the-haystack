use anyhow::{anyhow, Result};

use crate::needle::{
    location::Location, number::variants::FloatVariant, Interpret, Needle, Recombobulate,
};

#[derive(Clone, Debug, PartialEq)]
pub enum LocationVariant {
    // Decimal degrees
    DecimalDegrees(FloatVariant, FloatVariant),
    DecimalMinutes(FloatVariant, FloatVariant),
    DecimalSeconds(FloatVariant, FloatVariant),
    // TODO: Add IntegerVariant versions too
    // TODO: Also swapped latitude and longitude versions
}

impl Recombobulate for LocationVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            LocationVariant::DecimalDegrees(lat, lon) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value,
                            lon_float.value,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalDegrees: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalDegrees: {:?}",
                        &self
                    ))
                }
            }
            LocationVariant::DecimalMinutes(lat, lon) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value / 60.0,
                            lon_float.value / 60.0,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalMinutes: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalMinutes: {:?}",
                        &self
                    ))
                }
            }
            LocationVariant::DecimalSeconds(lat, lon) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value / 3600.0,
                            lon_float.value / 3600.0,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalSeconds: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalSeconds: {:?}",
                        &self
                    ))
                }
            }
        }
    }
}

impl Interpret for LocationVariant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        // First, interpret as two floats, as these are the basis for most of our location formats
        // We'll need to do this by hand as we want two adjacent floats rather than just one
        // if let Ok(float_variants) = FloatVariant::interpret(data) {
        //     for variant in &float_variants {
        //         intepretations.push(LocationVariant::DecimalDegrees(variant.clone()));
        //         intepretations.push(LocationVariant::DecimalMinutes(variant.clone()));
        //         intepretations.push(LocationVariant::DecimalSeconds(variant.clone()));
        //     }
        // }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid LocationVariant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
}

// impl Recombobulate for TimestampVariant {
//     fn recombobulate(&self) -> Result<Needle> {
//         match self {
//             TimestampVariant::EpochSecs(v) => {
//                 if let Ok(Needle::Integer(integer)) = v.recombobulate() {
//                     Ok(Needle::Timestamp(Timestamp::from_epoch_secs(
//                         integer.value,
//                     )?))
//                 } else {
//                     Err(anyhow!(
//                         "Failed to recreate Needle::Timestamp from epoch secs"
//                     ))
//                 }
//             }
//             TimestampVariant::EpochMillis(v) => {
//                 if let Ok(Needle::Integer(integer)) = v.recombobulate() {
//                     Ok(Needle::Timestamp(Timestamp::from_epoch_millis(
//                         integer.value,
//                     )?))
//                 } else {
//                     Err(anyhow!(
//                         "Failed to recreate Needle::Timestamp from epoch millis"
//                     ))
//                 }
//             }
//             TimestampVariant::EpochMicros(v) => {
//                 if let Ok(Needle::Integer(integer)) = v.recombobulate() {
//                     Ok(Needle::Timestamp(Timestamp::from_epoch_micros(
//                         integer.value,
//                     )?))
//                 } else {
//                     Err(anyhow!(
//                         "Failed to recreate Needle::Timestamp from epoch micros"
//                     ))
//                 }
//             }
//             TimestampVariant::EpochNanos(v) => {
//                 if let Ok(Needle::Integer(integer)) = v.recombobulate() {
//                     Ok(Needle::Timestamp(Timestamp::from_epoch_nanos(
//                         integer.value,
//                     )?))
//                 } else {
//                     Err(anyhow!(
//                         "Failed to recreate Needle::Timestamp from epoch nanos"
//                     ))
//                 }
//             }
//             TimestampVariant::DOSTime(v) => {
//                 if let Ok(Needle::Integer(integer)) = v.recombobulate() {
//                     if (u32::MIN as i64..=u32::MAX as i64).contains(&integer.value) {
//                         Ok(Needle::Timestamp(Timestamp::from_dos_time(
//                             integer.value as u32,
//                         )?))
//                     } else {
//                         Err(anyhow!(
//                             "Failed to recreate Needle::Timestamp from DOS time"
//                         ))
//                     }
//                 } else {
//                     Err(anyhow!(
//                         "Failed to recreate Needle::Timestamp from DOS time"
//                     ))
//                 }
//             }
//         }
//     }
// }
