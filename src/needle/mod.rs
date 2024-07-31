pub mod ipaddr;
pub mod location;
pub mod macaddr;
pub mod number;
pub mod timestamp;
pub mod variant;

use std::net::Ipv4Addr;

use anyhow::{anyhow, Result};
use location::variant::LocationVariant;
use measurements::Distance;
use time::{format_description, Duration, PrimitiveDateTime};

use self::{
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::{variants::TimestampVariant, Timestamp},
    variant::NeedleVariant,
};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Needle {
    Bytes(Vec<u8>),
    Integer(number::Integer),
    Float(number::Float),
    Timestamp(timestamp::Timestamp),
    Location(location::Location),
    IpAddr(ipaddr::IPv4),
    MacAddr(macaddr::MACAddr),
}

impl Needle {
    // Bytes creation

    // Integer creation
    pub fn new_integer(value: i64) -> Result<Self> {
        Ok(Needle::Integer(number::Integer::new(value)))
    }

    pub fn new_integer_with_tolerance(value: i64, tolerance: i64) -> Result<Self> {
        Ok(Needle::Integer(number::Integer::with_tolerance(
            value, tolerance,
        )))
    }

    // Float creation
    pub fn new_float(value: f64) -> Result<Self> {
        Ok(Needle::Float(number::Float::new(value)))
    }

    pub fn new_float_with_tolerance(value: f64, tolerance: f64) -> Result<Self> {
        Ok(Needle::Float(number::Float::with_tolerance(
            value, tolerance,
        )))
    }

    // Timestamp creation
    pub fn new_timestamp(dtg: &str) -> Result<Self> {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        if let Ok(datetime) = PrimitiveDateTime::parse(dtg, &format) {
            Ok(Self::Timestamp(Timestamp::new(datetime)))
        } else {
            Err(anyhow!("Failed to parse timestamp string: {}", dtg))
        }
    }

    pub fn new_timestamp_with_tolerance(dtg: &str, tolerance: Duration) -> Result<Self> {
        let format =
            format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

        if let Ok(datetime) = PrimitiveDateTime::parse(dtg, &format) {
            Ok(Self::Timestamp(Timestamp::with_tolerance(
                datetime, tolerance,
            )))
        } else {
            Err(anyhow!("Failed to parse timestamp string: {}", dtg))
        }
    }

    // Location creation
    pub fn new_location(lat: f64, lon: f64) -> Result<Self> {
        Ok(Needle::Location(location::Location::new(lat, lon)?))
    }

    pub fn new_location_with_tolerance(lat: f64, lon: f64, tolerance: Distance) -> Result<Self> {
        Ok(Needle::Location(location::Location::with_tolerance(
            lat, lon, tolerance,
        )?))
    }

    // IP Address creation
    pub fn new_ip_address(ipaddr: Ipv4Addr) -> Result<Self> {
        Ok(Needle::IpAddr(ipaddr::IPv4::new(ipaddr)?))
    }

    pub fn new_ip_address_with_tolerance(ipaddr: Ipv4Addr, cidr_prefix: u8) -> Result<Self> {
        Ok(Needle::IpAddr(ipaddr::IPv4::with_tolerance(
            ipaddr,
            cidr_prefix,
        )?))
    }

    // MAC Address creation
    // pub fn new_mac_address(dtg: &str) -> Result<Self> {
    //     todo!()
    // }

    // pub fn new_mac_address_with_tolerance(dtg: &str, tolerance: u8) -> Result<Self> {
    //     todo!()
    // }
}

pub trait Matches {
    fn matches(&self, rhs: &Self) -> bool;
}

impl Matches for Needle {
    fn matches(&self, rhs: &Needle) -> bool {
        match (&self, &rhs) {
            (Needle::Timestamp(lhs), Needle::Timestamp(rhs)) => lhs.matches(rhs),
            (Needle::Location(lhs), Needle::Location(rhs)) => lhs.matches(rhs),
            (Needle::IpAddr(lhs), Needle::IpAddr(rhs)) => lhs.matches(rhs),
            (Needle::MacAddr(lhs), Needle::MacAddr(rhs)) => lhs == rhs,
            (Needle::Integer(lhs), Needle::Integer(rhs)) => lhs.matches(rhs),
            (Needle::Float(lhs), Needle::Float(rhs)) => lhs.matches(rhs),
            (Needle::Bytes(lhs), Needle::Bytes(rhs)) => lhs == rhs,
            _ => false,
        }
    }
}

/// Trait for tranforming a Needle into all possible NeedleVariant byte sequences
pub trait Discombobulate {
    fn discombobulate(&self) -> Vec<NeedleVariant>;
}

impl Discombobulate for Needle {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        //Vec<(Vec<u8>, String)> {
        match &self {
            Needle::Timestamp(timestamp) => timestamp.discombobulate(),
            Needle::Location(location) => location.discombobulate(),
            Needle::IpAddr(_) => todo!(),
            Needle::MacAddr(_) => todo!(),
            Needle::Integer(integer) => integer.discombobulate(),
            Needle::Float(float) => float.discombobulate(),
            Needle::Bytes(_) => todo!(),
        }
    }
}

/// Trait for tranforming a NeedleVariant back into its Needle
pub trait Recombobulate {
    fn recombobulate(&self) -> Result<Needle>;
}

/// Trait for generating all possible valid interpretations from a byte sequence
pub trait Interpret {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized;
}

impl Interpret for Needle {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut needles = Vec::<Needle>::new();

        // Try all valid IntegerVariant interpretations
        if let Ok(integer_variants) = IntegerVariant::interpret(data) {
            for variant in &integer_variants {
                if let Ok(needle) = variant.recombobulate() {
                    println!("{:02x?} -> {:?}", &variant, &needle);
                    needles.push(needle);
                }
            }
        }

        // Try all valid FloatVariant interpretations
        if let Ok(float_variants) = FloatVariant::interpret(data) {
            for variant in &float_variants {
                if let Ok(needle) = variant.recombobulate() {
                    println!("{:02x?} -> {:?}", &variant, &needle);
                    needles.push(needle);
                }
            }
        }

        // Try all valid TimestampVariant interpretations
        if let Ok(timestamp_variants) = TimestampVariant::interpret(data) {
            for variant in &timestamp_variants {
                if let Ok(needle) = variant.recombobulate() {
                    println!("{:02x?} -> {:?}", &variant, &needle);
                    needles.push(needle);
                }
            }
        }

        // Try all valid LocationVariant interpretations
        if let Ok(location_variants) = LocationVariant::interpret(data) {
            for variant in &location_variants {
                if let Ok(needle) = variant.recombobulate() {
                    println!("{:02x?} -> {:?}", &variant, &needle);
                    needles.push(needle);
                }
            }
        }

        if needles.is_empty() {
            Err(anyhow!("Failed to interpret bytes as any valid Needles!"))
        } else {
            needles.sort_by(|a, b| a.partial_cmp(b).unwrap());
            needles.dedup();
            Ok(needles)
        }
    }
}

#[cfg(test)]
mod tests {

    use time::Duration;

    use crate::needle::{location::Location, number::*, Discombobulate, Matches, Needle};

    use super::Interpret;

    #[test]
    fn new_timestamps() {
        let needle = Needle::new_timestamp("2023-12-31 23:59:58").unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_zero() {
        let needle = Needle::new_integer(0).unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_negative() {
        let needle = Needle::new_integer(-3).unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_positive() {
        let needle = Needle::new_integer(12345).unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_zero() {
        let needle = Needle::new_float(0.0).unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_negative() {
        let needle = Needle::new_float(-1.0).unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_positive() {
        let needle = Needle::new_float(2.2).unwrap();
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
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
            //println!("{:>20} : {:02x?}", variant.1, variant.0);
            println!("{:02x?}", variant);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn interpret_integer() {
        let data = vec![0xf0u8, 0x01, 0x00, 0x00];

        if let Ok(interpretations) = Needle::interpret(&data) {
            println!("{:#?}", &interpretations);
        }
    }

    #[test]
    fn interpret_location() {
        let data = vec![0x9au8, 0xd9, 0x1d, 0xc3, 0xe1, 0x7a, 0xaa, 0x41];

        if let Ok(interpretations) = Needle::interpret(&data) {
            println!("{:#?}", &interpretations);
        }
    }

    #[test]
    fn matches_ipaddr() {
        // Exactly the same
        let lhs = Needle::new_ip_address("192.168.0.1".parse().unwrap()).unwrap();
        let rhs = Needle::new_ip_address("192.168.0.1".parse().unwrap()).unwrap();

        assert!(lhs.matches(&rhs));

        // Not the same
        let lhs = Needle::new_ip_address("192.168.0.1".parse().unwrap()).unwrap();
        let rhs = Needle::new_ip_address("192.168.1.1".parse().unwrap()).unwrap();

        assert!(!lhs.matches(&rhs));

        // Within the same /24
        let lhs = Needle::new_ip_address("192.168.0.1".parse().unwrap()).unwrap();
        let rhs =
            Needle::new_ip_address_with_tolerance("192.168.0.0".parse().unwrap(), 24).unwrap();

        assert!(lhs.matches(&rhs));

        // Within the same /16
        let lhs = Needle::new_ip_address("192.168.22.33".parse().unwrap()).unwrap();
        let rhs =
            Needle::new_ip_address_with_tolerance("192.168.0.0".parse().unwrap(), 16).unwrap();

        assert!(lhs.matches(&rhs));

        // Not within the same /16
        let lhs = Needle::new_ip_address("192.168.22.33".parse().unwrap()).unwrap();
        let rhs = Needle::new_ip_address_with_tolerance("192.0.0.0".parse().unwrap(), 16).unwrap();

        assert!(!lhs.matches(&rhs));

        // Within the same /8
        let lhs = Needle::new_ip_address("192.1.2.3".parse().unwrap()).unwrap();
        let rhs =
            Needle::new_ip_address_with_tolerance("192.255.255.255".parse().unwrap(), 8).unwrap();

        assert!(lhs.matches(&rhs));
    }
}
