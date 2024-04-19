pub mod variant;

use anyhow::{anyhow, Result};
use geo::{HaversineDistance, Point};
use itertools::Itertools;
use measurements::Distance;

use self::variant::LocationVariant::*;

use super::{
    number::variants::FloatVariant, variant::NeedleVariant, Discombobulate, Matches, Needle,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    value: Point,
    tolerance: Option<Distance>,
}

// TODO: Fix this so it compares points better
impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value.x().partial_cmp(&other.value.x()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.tolerance.partial_cmp(&other.tolerance)
    }
}

impl Location {
    pub fn new(lat: f64, lon: f64) -> Result<Self> {
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            Err(anyhow!("Invalid lat/lon values: {},{}", lat, lon))
        } else {
            Ok(Self {
                value: Point::new(lon, lat), // A Point takes an x and a y, hence lon then lat rather than lat then lon
                tolerance: None,
            })
        }
    }

    pub fn with_tolerance(lat: f64, lon: f64, tolerance: Distance) -> Result<Self> {
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            Err(anyhow!("Invalid lat/lon values: {},{}", lat, lon))
        } else {
            Ok(Self {
                value: Point::new(lon, lat),
                tolerance: Some(tolerance),
            })
        }
    }
}

impl Matches for Location {
    fn matches(&self, rhs: &Self) -> bool {
        // If rhs has a tolerance, check that lhs falls wthin it
        match rhs.tolerance {
            Some(tolerance) => {
                let actual_difference = self.value.haversine_distance(&rhs.value) as u64;

                // println!("Actual dif: {}", actual_difference);
                // println!("Allowed dif: {}", tolerance);

                actual_difference <= tolerance.as_meters() as u64
            }
            None => self.value == rhs.value,
        }
    }
}

impl Discombobulate for Location {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        // Decimal degrees
        // ---------------
        let (lon_decimal_degrees, lat_decimal_degrees) = self.value.x_y();

        // First, turn the latitude and longitude into Floats as these are the basis for most of our location formats
        let lat_needle_variants = if let Ok(float_needle) = Needle::new_float(lat_decimal_degrees) {
            float_needle.discombobulate()
        } else {
            Vec::<NeedleVariant>::new()
        };

        let lon_needle_variants = if let Ok(float_needle) = Needle::new_float(lon_decimal_degrees) {
            float_needle.discombobulate()
        } else {
            Vec::<NeedleVariant>::new()
        };

        // TODO: Make a more intelligent assessment here
        if lat_needle_variants.len() != lon_needle_variants.len() {
            println!(
                "{} vs {} lat and lon needle variants!",
                lat_needle_variants.len(),
                lon_needle_variants.len()
            );
            return variants;
        }

        // Zip together the two vectors of NeedleVariants and only keep the Floats
        let lat_lon_needle_variants = lat_needle_variants
            .iter()
            .zip(lon_needle_variants.iter())
            .filter(|(lat_needle_variant, lon_needle_variant)| {
                matches!(
                    (lat_needle_variant, lon_needle_variant),
                    (NeedleVariant::Float(_), NeedleVariant::Float(_),)
                )
            })
            .collect_vec();

        // Pair up matching FloatVariants
        for (lat_variants, lon_variants) in &lat_lon_needle_variants {
            if let (
                NeedleVariant::Float(lat_float_variant),
                NeedleVariant::Float(lon_float_variant),
            ) = (lat_variants, lon_variants)
            {
                match (lat_float_variant, lon_float_variant) {
                    (FloatVariant::F32LE(_), FloatVariant::F32LE(_))
                    | (FloatVariant::F32BE(_), FloatVariant::F32BE(_))
                    | (FloatVariant::F64LE(_), FloatVariant::F64LE(_))
                    | (FloatVariant::F64BE(_), FloatVariant::F64BE(_)) => {
                        variants.push(NeedleVariant::Location(DecimalDegrees(
                            lat_float_variant.clone(),
                            lon_float_variant.clone(),
                        )));
                    }
                    _ => (),
                }
            }
        }

        // Decimal minutes
        // ---------------
        let lat_decimal_minutes = lat_decimal_degrees * 60.0;
        let lon_decimal_minutes = lon_decimal_degrees * 60.0;

        // First, turn the latitude and longitude into Needle::Float
        let lat_needle_variants = if let Ok(float_needle) = Needle::new_float(lat_decimal_minutes) {
            float_needle.discombobulate()
        } else {
            Vec::<NeedleVariant>::new()
        };

        let lon_needle_variants = if let Ok(float_needle) = Needle::new_float(lon_decimal_minutes) {
            float_needle.discombobulate()
        } else {
            Vec::<NeedleVariant>::new()
        };

        // TODO: Make a more intelligent assessment here
        if lat_needle_variants.len() != lon_needle_variants.len() {
            println!(
                "{} vs {} lat and lon needle variants!",
                lat_needle_variants.len(),
                lon_needle_variants.len()
            );
            return variants;
        }

        // Zip together the two vectors of NeedleVariants and only keep NeedleVariant::Float
        let lat_lon_needle_variants = lat_needle_variants
            .iter()
            .zip(lon_needle_variants.iter())
            .filter(|(lat_needle_variant, lon_needle_variant)| {
                matches!(
                    (lat_needle_variant, lon_needle_variant),
                    (NeedleVariant::Float(_), NeedleVariant::Float(_),)
                )
            })
            .collect_vec();

        // Keep matching pairs of FloatVariant
        for (lat_variants, lon_variants) in &lat_lon_needle_variants {
            if let (
                NeedleVariant::Float(lat_float_variant),
                NeedleVariant::Float(lon_float_variant),
            ) = (lat_variants, lon_variants)
            {
                match (lat_float_variant, lon_float_variant) {
                    (FloatVariant::F32LE(_), FloatVariant::F32LE(_))
                    | (FloatVariant::F32BE(_), FloatVariant::F32BE(_))
                    | (FloatVariant::F64LE(_), FloatVariant::F64LE(_))
                    | (FloatVariant::F64BE(_), FloatVariant::F64BE(_)) => {
                        variants.push(NeedleVariant::Location(DecimalMinutes(
                            lat_float_variant.clone(),
                            lon_float_variant.clone(),
                        )));
                    }
                    _ => (),
                }
            }
        }

        // Decimal seconds
        // ---------------
        let lat_decimal_seconds = lat_decimal_minutes * 60.0;
        let lon_decimal_seconds = lon_decimal_minutes * 60.0;

        // First, turn the latitude and longitude into Needle::Float
        let lat_needle_variants = if let Ok(float_needle) = Needle::new_float(lat_decimal_seconds) {
            float_needle.discombobulate()
        } else {
            Vec::<NeedleVariant>::new()
        };

        let lon_needle_variants = if let Ok(float_needle) = Needle::new_float(lon_decimal_seconds) {
            float_needle.discombobulate()
        } else {
            Vec::<NeedleVariant>::new()
        };

        // TODO: Make a more intelligent assessment here
        if lat_needle_variants.len() != lon_needle_variants.len() {
            println!(
                "{} vs {} lat and lon needle variants!",
                lat_needle_variants.len(),
                lon_needle_variants.len()
            );
            return variants;
        }

        // Zip together the two vectors of NeedleVariants and only keep NeedleVariant::Float
        let lat_lon_needle_variants = lat_needle_variants
            .iter()
            .zip(lon_needle_variants.iter())
            .filter(|(lat_needle_variant, lon_needle_variant)| {
                matches!(
                    (lat_needle_variant, lon_needle_variant),
                    (NeedleVariant::Float(_), NeedleVariant::Float(_),)
                )
            })
            .collect_vec();

        // Keep matching pairs of FloatVariant
        for (lat_variants, lon_variants) in &lat_lon_needle_variants {
            if let (
                NeedleVariant::Float(lat_float_variant),
                NeedleVariant::Float(lon_float_variant),
            ) = (lat_variants, lon_variants)
            {
                match (lat_float_variant, lon_float_variant) {
                    (FloatVariant::F32LE(_), FloatVariant::F32LE(_))
                    | (FloatVariant::F32BE(_), FloatVariant::F32BE(_))
                    | (FloatVariant::F64LE(_), FloatVariant::F64LE(_))
                    | (FloatVariant::F64BE(_), FloatVariant::F64BE(_)) => {
                        variants.push(NeedleVariant::Location(DecimalSeconds(
                            lat_float_variant.clone(),
                            lon_float_variant.clone(),
                        )));
                    }
                    _ => (),
                }
            }
        }

        // TODO: add support for:
        // Degrees decimal minutes
        // Degrees Minutes Seconds
        //

        variants
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn location_test() {
        let empire_state_building = Location::new(40.7484, -73.9856).unwrap();
        let washington_monument = Location::new(38.8894, -77.0352).unwrap();

        let metres_between = empire_state_building
            .value
            .haversine_distance(&washington_monument.value) as u64;

        let km_between = metres_between / 1000;

        println!(
            "{:?} -> {:?} == {} m ({} km)",
            empire_state_building, washington_monument, metres_between, km_between
        );

        assert_eq!(km_between, 332);

        // And in the other direction

        let metres_between = washington_monument
            .value
            .haversine_distance(&empire_state_building.value) as u64;

        let km_between = metres_between / 1000;

        println!(
            "{:?} -> {:?} == {} m ({} km)",
            washington_monument, empire_state_building, metres_between, km_between
        );

        assert_eq!(km_between, 332);
    }

    #[test]
    fn location_with_tolerance_test() {
        let p1 = Location::new(38.88929, -77.04824).unwrap();
        let p2 =
            Location::with_tolerance(38.88940, -77.04111, Distance::from_kilometres(1.0)).unwrap();

        assert!(p1.matches(&p2));
    }

    #[test]
    fn location_discombobulation() {
        let location = Location::new(38.88929, -77.04824).unwrap();

        let variants = location.discombobulate();
        for variant in &variants {
            println!("{:?}", variant);
        }
    }
}
