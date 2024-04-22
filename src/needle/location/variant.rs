use anyhow::{anyhow, Result};

use crate::needle::{
    location::Location, number::variants::FloatVariant, Interpret, Needle, Recombobulate,
};

#[derive(Clone, Debug, PartialEq)]
pub enum LocationVariant {
    // Decimal degrees
    DecimalDegreesLatLon(FloatVariant, FloatVariant),
    DecimalDegreesLonLat(FloatVariant, FloatVariant),

    // Decimal minutes
    DecimalMinutesLatLon(FloatVariant, FloatVariant),
    DecimalMinutesLonLat(FloatVariant, FloatVariant),

    // Decimal seconds
    DecimalSecondsLatLon(FloatVariant, FloatVariant),
    DecimalSecondsLonLat(FloatVariant, FloatVariant),
    // TODO: Add IntegerVariant versions too
}

impl Recombobulate for LocationVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            LocationVariant::DecimalDegreesLatLon(lat, lon) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value,
                            lon_float.value,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalDegreesLatLon: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalDegreesLatLon: {:?}",
                        &self
                    ))
                }
            }

            LocationVariant::DecimalDegreesLonLat(lon, lat) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value,
                            lon_float.value,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalDegreesLonLat: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalDegreesLonLat: {:?}",
                        &self
                    ))
                }
            }

            LocationVariant::DecimalMinutesLatLon(lat, lon) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value / 60.0,
                            lon_float.value / 60.0,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalMinutesLatLon: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalMinutesLatLon: {:?}",
                        &self
                    ))
                }
            }

            LocationVariant::DecimalMinutesLonLat(lon, lat) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value / 60.0,
                            lon_float.value / 60.0,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalMinutesLonLat: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalMinutesLonLat: {:?}",
                        &self
                    ))
                }
            }

            LocationVariant::DecimalSecondsLatLon(lat, lon) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value / 3600.0,
                            lon_float.value / 3600.0,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalSecondsLatLon: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalSecondsLatLon: {:?}",
                        &self
                    ))
                }
            }

            LocationVariant::DecimalSecondsLonLat(lon, lat) => {
                if let Ok(Needle::Float(lat_float)) = lat.recombobulate() {
                    if let Ok(Needle::Float(lon_float)) = lon.recombobulate() {
                        Ok(Needle::Location(Location::new(
                            lat_float.value / 3600.0,
                            lon_float.value / 3600.0,
                        )?))
                    } else {
                        Err(anyhow!(
                            "Failed to recreate Needle::Location from LocationVariant::DecimalSecondsLonLat: {:?}",
                            &self
                        ))
                    }
                } else {
                    Err(anyhow!(
                        "Failed to recreate Needle::Location from LocationVariant::DecimalSecondsLonLat: {:?}",
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
        // For now we'll assume both floats have the same endianness

        // Get two 32 bit floats, but only if both are valid
        if data.len() >= 8 {
            if let Ok(f1) = FloatVariant::as_f32_le(data) {
                if let Ok(f2) = FloatVariant::as_f32_le(data.split_at(4).1) {
                    // Decimal degrees
                    intepretations.push(LocationVariant::DecimalDegreesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalDegreesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal minutes
                    intepretations.push(LocationVariant::DecimalMinutesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalMinutesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal seconds
                    intepretations.push(LocationVariant::DecimalSecondsLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalSecondsLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));
                }
            }

            if let Ok(f1) = FloatVariant::as_f32_be(data) {
                if let Ok(f2) = FloatVariant::as_f32_be(data.split_at(4).1) {
                    // Decimal degrees
                    intepretations.push(LocationVariant::DecimalDegreesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalDegreesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal minutes
                    intepretations.push(LocationVariant::DecimalMinutesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalMinutesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal seconds
                    intepretations.push(LocationVariant::DecimalSecondsLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalSecondsLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));
                }
            }
        }

        // Get two 64 bit floats
        if data.len() >= 16 {
            if let Ok(f1) = FloatVariant::as_f64_le(data) {
                if let Ok(f2) = FloatVariant::as_f64_le(data.split_at(8).1) {
                    // Decimal degrees
                    intepretations.push(LocationVariant::DecimalDegreesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalDegreesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal minutes
                    intepretations.push(LocationVariant::DecimalMinutesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalMinutesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal seconds
                    intepretations.push(LocationVariant::DecimalSecondsLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalSecondsLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));
                }
            }

            if let Ok(f1) = FloatVariant::as_f64_be(data) {
                if let Ok(f2) = FloatVariant::as_f64_be(data.split_at(8).1) {
                    // Decimal degrees
                    intepretations.push(LocationVariant::DecimalDegreesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalDegreesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal minutes
                    intepretations.push(LocationVariant::DecimalMinutesLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalMinutesLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));

                    // Decimal seconds
                    intepretations.push(LocationVariant::DecimalSecondsLatLon(
                        f1.clone(),
                        f2.clone(),
                    ));
                    intepretations.push(LocationVariant::DecimalSecondsLonLat(
                        f1.clone(),
                        f2.clone(),
                    ));
                }
            }
        }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid LocationVariant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
}

#[cfg(test)]
mod tests {

    use measurements::Distance;

    use crate::needle::Matches;

    use super::*;

    #[test]
    fn location_variants() {
        // Coords: 21.31, -157.85
        let data = vec![0x41u8, 0xaa, 0x7a, 0xe1, 0xc3, 0x1d, 0xd9, 0x9a]; // DecimalDegreesLatLon(F32BE)

        let target =
            Needle::new_location_with_tolerance(21.305, -157.845, Distance::from_kilometres(1.0))
                .unwrap(); // Honolulu

        let interps = LocationVariant::interpret(&data);
        assert!(interps.is_ok());

        for location_variant in interps.as_ref().unwrap() {
            // println!("{:?}", &location_variant);

            if let Ok(location) = location_variant.recombobulate() {
                // println!("{:?}", &location);

                if location.matches(&target) {
                    println!("[ Target needle found ]");
                    println!("Target  : {:?}", target);
                    println!("Actual  : {:?}", location);
                    println!("Variant : {:02x?}", location_variant);
                }
            }
        }

        assert!(interps.unwrap()[6]
            .recombobulate()
            .unwrap()
            .matches(&target));

        let data = vec![
            0x00u8, 0x00, 0x00, 0x00, 0x88, 0x57, 0x21, 0xc1, 0x00, 0x00, 0x00, 0x00, 0xc0, 0xba,
            0xf2, 0x40,
        ]; // DecimalSecondsLonLat(F64LE)

        let interps = LocationVariant::interpret(&data);
        assert!(interps.is_ok());

        for location_variant in interps.as_ref().unwrap() {
            // println!("{:?}", &location_variant);

            if let Ok(location) = location_variant.recombobulate() {
                // println!("{:?}", &location);

                if location.matches(&target) {
                    println!("[ Target needle found ]");
                    println!("Target  : {:?}", target);
                    println!("Actual  : {:?}", location);
                    println!("Variant : {:02x?}", location_variant);
                }
            }
        }

        assert!(interps.unwrap()[17]
            .recombobulate()
            .unwrap()
            .matches(&target));

        // Coords: 21.31, -157.85
        // Location(DecimalDegreesLatLon(F64LE(([8f, c2, f5, 28, 5c, 4f, 35, 40], 21.31)), F64LE(([33, 33, 33, 33, 33, bb, 63, c0], -157.85))))
        // Location(DecimalDegreesLonLat(F64LE(([33, 33, 33, 33, 33, bb, 63, c0], -157.85)), F64LE(([8f, c2, f5, 28, 5c, 4f, 35, 40], 21.31))))
        // Location(DecimalDegreesLatLon(F64BE(([40, 35, 4f, 5c, 28, f5, c2, 8f], 21.31)), F64BE(([c0, 63, bb, 33, 33, 33, 33, 33], -157.85))))
        // Location(DecimalDegreesLonLat(F64BE(([c0, 63, bb, 33, 33, 33, 33, 33], -157.85)), F64BE(([40, 35, 4f, 5c, 28, f5, c2, 8f], 21.31))))
        // Location(DecimalDegreesLatLon(F32LE(([e1, 7a, aa, 41], 21.31)), F32LE(([9a, d9, 1d, c3], -157.85))))
        // Location(DecimalDegreesLonLat(F32LE(([9a, d9, 1d, c3], -157.85)), F32LE(([e1, 7a, aa, 41], 21.31))))
        // Location(DecimalDegreesLatLon(F32BE(([41, aa, 7a, e1], 21.31)), F32BE(([c3, 1d, d9, 9a], -157.85))))
        // Location(DecimalDegreesLonLat(F32BE(([c3, 1d, d9, 9a], -157.85)), F32BE(([41, aa, 7a, e1], 21.31))))
        // Location(DecimalMinutesLatLon(F64LE(([66, 66, 66, 66, 66, fa, 93, 40], 1278.6)), F64LE(([00, 00, 00, 00, 80, 7f, c2, c0], -9471.0))))
        // Location(DecimalMinutesLonLat(F64LE(([00, 00, 00, 00, 80, 7f, c2, c0], -9471.0)), F64LE(([66, 66, 66, 66, 66, fa, 93, 40], 1278.6))))
        // Location(DecimalMinutesLatLon(F64BE(([40, 93, fa, 66, 66, 66, 66, 66], 1278.6)), F64BE(([c0, c2, 7f, 80, 00, 00, 00, 00], -9471.0))))
        // Location(DecimalMinutesLonLat(F64BE(([c0, c2, 7f, 80, 00, 00, 00, 00], -9471.0)), F64BE(([40, 93, fa, 66, 66, 66, 66, 66], 1278.6))))
        // Location(DecimalMinutesLatLon(F32LE(([33, d3, 9f, 44], 1278.6)), F32LE(([00, fc, 13, c6], -9471.0))))
        // Location(DecimalMinutesLonLat(F32LE(([00, fc, 13, c6], -9471.0)), F32LE(([33, d3, 9f, 44], 1278.6))))
        // Location(DecimalMinutesLatLon(F32BE(([44, 9f, d3, 33], 1278.6)), F32BE(([c6, 13, fc, 00], -9471.0))))
        // Location(DecimalMinutesLonLat(F32BE(([c6, 13, fc, 00], -9471.0)), F32BE(([44, 9f, d3, 33], 1278.6))))
        // Location(DecimalSecondsLatLon(F64LE(([00, 00, 00, 00, c0, ba, f2, 40], 76716.0)), F64LE(([00, 00, 00, 00, 88, 57, 21, c1], -568260.0))))
        // Location(DecimalSecondsLonLat(F64LE(([00, 00, 00, 00, 88, 57, 21, c1], -568260.0)), F64LE(([00, 00, 00, 00, c0, ba, f2, 40], 76716.0))))
        // Location(DecimalSecondsLatLon(F64BE(([40, f2, ba, c0, 00, 00, 00, 00], 76716.0)), F64BE(([c1, 21, 57, 88, 00, 00, 00, 00], -568260.0))))
        // Location(DecimalSecondsLonLat(F64BE(([c1, 21, 57, 88, 00, 00, 00, 00], -568260.0)), F64BE(([40, f2, ba, c0, 00, 00, 00, 00], 76716.0))))
        // Location(DecimalSecondsLatLon(F32LE(([00, d6, 95, 47], 76716.0)), F32LE(([40, bc, 0a, c9], -568260.0))))
        // Location(DecimalSecondsLonLat(F32LE(([40, bc, 0a, c9], -568260.0)), F32LE(([00, d6, 95, 47], 76716.0))))
        // Location(DecimalSecondsLatLon(F32BE(([47, 95, d6, 00], 76716.0)), F32BE(([c9, 0a, bc, 40], -568260.0))))
        // Location(DecimalSecondsLonLat(F32BE(([c9, 0a, bc, 40], -568260.0)), F32BE(([47, 95, d6, 00], 76716.0))))
    }
}
