use time::{format_description, Duration, PrimitiveDateTime};

use self::timestamp::Timestamp;

pub mod ipaddr;
pub mod location;
pub mod macaddr;
pub mod number;
pub mod timestamp;

#[derive(Clone, Debug, PartialEq)]
pub enum Needle {
    Timestamp(timestamp::Timestamp),
    Location(location::Location),
    IpAddr(ipaddr::IPv4),
    MacAddr(macaddr::MACAddr),
    Integer(number::Integer),
    Float(number::Float),
    Bytes(Vec<u8>),
}

impl Needle {
    // Timestamp creation
    pub fn new_timestamp(dtg: &str) -> Option<Self> {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        if let Ok(datetime) = PrimitiveDateTime::parse(dtg, &format) {
            Some(Self::Timestamp(Timestamp::new(datetime)))
        } else {
            None
        }
    }

    pub fn new_timestamp_with_tolerance(dtg: &str, tolerance: Duration) -> Option<Self> {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        if let Ok(datetime) = PrimitiveDateTime::parse(dtg, &format) {
            Some(Self::Timestamp(Timestamp::with_tolerance(
                datetime, tolerance,
            )))
        } else {
            None
        }
    }

    // Number creation
    pub fn new_integer(value: i64) -> Option<Self> {
        Some(Needle::Integer(number::Integer::new(value)))
    }

    pub fn new_integer_with_tolerance(value: i64, tolerance: i64) -> Option<Self> {
        Some(Needle::Integer(number::Integer::with_tolerance(
            value, tolerance,
        )))
    }

    pub fn new_float(value: f64) -> Option<Self> {
        Some(Needle::Float(number::Float::new(value)))
    }

    pub fn new_float_with_tolerance(value: f64, tolerance: f64) -> Option<Self> {
        Some(Needle::Float(number::Float::with_tolerance(
            value, tolerance,
        )))
    }
}

pub trait Matches {
    fn matches(&self, rhs: &Self) -> bool;
}

impl Needle {
    pub fn matches(&self, rhs: &Needle) -> bool {
        match (&self, &rhs) {
            (Needle::Timestamp(lhs), Needle::Timestamp(rhs)) => lhs.matches(rhs),
            (Needle::Location(lhs), Needle::Location(rhs)) => lhs.matches(rhs),
            (Needle::IpAddr(lhs), Needle::IpAddr(rhs)) => lhs == rhs,
            (Needle::MacAddr(lhs), Needle::MacAddr(rhs)) => lhs == rhs,
            (Needle::Integer(lhs), Needle::Integer(rhs)) => lhs.matches(rhs),
            (Needle::Float(lhs), Needle::Float(rhs)) => lhs.matches(rhs),
            (Needle::Bytes(lhs), Needle::Bytes(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

pub trait Discombobulate {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)>; // TODO: Change to HashMap
}

impl Discombobulate for Needle {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        match &self {
            Needle::Timestamp(timestamp) => timestamp.discombobulate(),
            Needle::Location(location) => location.discombobulate(),
            Needle::IpAddr(_) => todo!(),
            Needle::MacAddr(_) => todo!(),
            Needle::Integer(integer) => integer.discombobulate(),
            Needle::Float(float) => float.discombobulate(),
            Needle::Bytes(bytes) => vec![(bytes.to_vec(), String::from("Byte sequence"))],
        }
    }
}

#[cfg(test)]
mod tests {

    use time::{macros::datetime, Duration};

    use crate::needle::{
        location::Location,
        number::{self, *},
        timestamp::Timestamp,
        Discombobulate, Needle,
    };

    #[test]
    fn new_timestamps() {
        let needle = Needle::new_timestamp("2023-12-31 23:59:58").unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_zero() {
        let needle = Needle::new_integer(0).unwrap(); //Needle::Integer(number::Integer::new(0));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_negative() {
        let needle = Needle::new_integer(-3).unwrap(); //Needle::Integer(number::Integer::new(-3));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_positive() {
        let needle = Needle::new_integer(12345).unwrap(); //Needle::Integer(number::Integer::new(12345));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_zero() {
        let needle = Needle::new_float(0.0).unwrap(); //Needle::Float(number::Float::new(0.0));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_negative() {
        let needle = Needle::new_float(-1.0).unwrap(); //Needle::Float(number::Float::new(-1.0));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_positive() {
        let needle = Needle::new_float(2.2).unwrap(); //Needle::Float(number::Float::new(2.2));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn matches_timestamp() {
        // lhs is 12 hours prior to rhs, with a tolerance of 1 day (so DOES match)
        let lhs = Needle::new_timestamp("2023-12-31 12:00:00").unwrap();

        let rhs =
            Needle::new_timestamp_with_tolerance("2024-01-01 00:00:00", Duration::days(1)).unwrap();

        assert!(lhs.matches(&rhs));

        // lhs is 1 full day after rhs, with a tolerance of 1 day (so DOES match)
        let lhs = Needle::new_timestamp("2024-01-02 00:00:00").unwrap();

        let rhs =
            Needle::new_timestamp_with_tolerance("2024-01-01 00:00:00", Duration::days(1)).unwrap();

        assert!(lhs.matches(&rhs));

        // lhs is 30 seconds prior to rhs, with a tolerance of 1 minute (so DOES match)
        let lhs = Needle::new_timestamp("2024-01-01 00:00:00").unwrap();

        let rhs = Needle::new_timestamp_with_tolerance("2024-01-01 00:00:30", Duration::minutes(1))
            .unwrap();

        assert!(lhs.matches(&rhs));

        // lhs is 5 seconds prior to rhs, with no tolerance (so does NOT match)
        let lhs = Needle::new_timestamp("2023-12-31 23:59:55").unwrap();
        let rhs = Needle::new_timestamp("2024-01-01 00:00:00").unwrap();

        assert!(!lhs.matches(&rhs));

        // lhs is exactly the same as rhs, with no tolerance (so DOES match)
        let lhs = Needle::new_timestamp("2024-01-01 00:00:00").unwrap();
        let rhs = Needle::new_timestamp("2024-01-01 00:00:00").unwrap();

        assert!(lhs.matches(&rhs));
    }

    #[test]
    fn matches_integer() {
        let lhs = Needle::Integer(Integer::new(0));
        let rhs = Needle::Integer(Integer::with_tolerance(10, 20));

        assert!(lhs.matches(&rhs));
    }

    #[test]
    fn location_zero() {
        let empire_state_building = Location::new(40.7484, -73.9856).unwrap();
        let needle = Needle::Location(empire_state_building);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }
}
