pub mod variant;

use geo::{HaversineDistance, Point};
use measurements::Distance;

use self::variant::LocationVariant::*;

use super::{variant::NeedleVariant, Discombobulate, Matches};

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
    pub fn new(lat: f64, lon: f64) -> Option<Self> {
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            None
        } else {
            Some(Self {
                value: Point::new(lon, lat), // A Point takes an x and a y, hence lon then lat rather than lat then lon
                tolerance: None,
            })
        }
    }

    pub fn with_tolerance(lat: f64, lon: f64, tolerance: Distance) -> Option<Self> {
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            None
        } else {
            Some(Self {
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

        let (lon, lat) = self.value.x_y();

        // Decimal degrees
        let dd_f64_le = [lat.to_le_bytes().as_slice(), lon.to_le_bytes().as_slice()].concat();
        let dd_f64_be = [lat.to_be_bytes().as_slice(), lon.to_be_bytes().as_slice()].concat();

        variants.push(NeedleVariant::Location(DecimalDegreesF64LE(dd_f64_le)));
        variants.push(NeedleVariant::Location(DecimalDegreesF64BE(dd_f64_be)));

        // Also try as an f32
        if (f32::MIN as f64..=f32::MAX as f64).contains(&lat)
            && (f32::MIN as f64..=f32::MAX as f64).contains(&lon)
        {
            let dd_f32_le = [
                (lat as f32).to_le_bytes().as_slice(),
                (lon as f32).to_le_bytes().as_slice(),
            ]
            .concat();
            let dd_f32_be = [
                (lat as f32).to_be_bytes().as_slice(),
                (lon as f32).to_be_bytes().as_slice(),
            ]
            .concat();

            variants.push(NeedleVariant::Location(DecimalDegreesF32LE(dd_f32_le)));
            variants.push(NeedleVariant::Location(DecimalDegreesF32BE(dd_f32_be)));
        }

        // TODO: add support for:
        // Degrees decimal minutes
        // Degrees Minutes Seconds as 3 x u8
        // Degrees Minutes Seconds as 1 x u16

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
}
