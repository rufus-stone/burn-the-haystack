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

// impl Needle {
//     pub fn new_timestamp() -> Self {
//         Self::Timestamp()
//     }
// }

/*
Do something like this to make needle creation cleaner

macro_rules! impl_varint {
    ($t:ty, unsigned) => {
        impl VarInt for $t {
            fn required_space(self) -> usize {
                required_encoded_space_unsigned(self as u64)
            }

            fn decode_var(src: &[u8]) -> Option<(Self, usize)> {
                let (n, s) = u64::decode_var(src)?;
                Some((n as Self, s))
            }

            fn encode_var(self, dst: &mut [u8]) -> usize {
                (self as u64).encode_var(dst)
            }
        }
    };
    ($t:ty, signed) => {
        impl VarInt for $t {
            fn required_space(self) -> usize {
                required_encoded_space_signed(self as i64)
            }

            fn decode_var(src: &[u8]) -> Option<(Self, usize)> {
                let (n, s) = i64::decode_var(src)?;
                Some((n as Self, s))
            }

            fn encode_var(self, dst: &mut [u8]) -> usize {
                (self as i64).encode_var(dst)
            }
        }
    };
}

impl_varint!(usize, unsigned);

*/

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

    //use super::{timestamp::Timestamp, *};

    #[test]
    fn integer_zero() {
        let needle = Needle::Integer(number::Integer::new(0)); //Needle::Integer(0);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_negative() {
        let needle = Needle::Integer(number::Integer::new(-3));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_positive() {
        let needle = Needle::Integer(number::Integer::new(12345));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_zero() {
        let needle = Needle::Float(number::Float::new(0.0));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_negative() {
        let needle = Needle::Float(number::Float::new(-1.0));
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_positive() {
        let needle = Needle::Float(number::Float::new(2.2));
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
        let lhs = Needle::Timestamp(Timestamp::new(datetime!(2023-12-31 12:00:00)));

        let rhs = Needle::Timestamp(Timestamp::with_tolerance(
            datetime!(2024-01-01 00:00:00),
            Duration::days(1),
        ));

        assert!(lhs.matches(&rhs));

        // lhs is 1 full day after rhs, with a tolerance of 1 day (so DOES match)
        let lhs = Needle::Timestamp(Timestamp::new(datetime!(2024-01-02 00:00:00)));

        let rhs = Needle::Timestamp(Timestamp::with_tolerance(
            datetime!(2024-01-01 00:00:00),
            Duration::days(1),
        ));

        assert!(lhs.matches(&rhs));

        // lhs is 30 seconds prior to rhs, with a tolerance of 1 minute (so DOES match)
        let lhs = Needle::Timestamp(Timestamp::new(datetime!(2024-01-01 00:00:00)));

        let rhs = Needle::Timestamp(Timestamp::with_tolerance(
            datetime!(2024-01-01 00:00:30),
            Duration::minutes(1),
        ));

        assert!(lhs.matches(&rhs));

        // lhs is 5 seconds prior to rhs, with no tolerance (so does NOT match)
        let lhs = Needle::Timestamp(Timestamp::new(datetime!(2023-12-31 23:59:55)));
        let rhs = Needle::Timestamp(Timestamp::new(datetime!(2024-01-01 00:00:00)));

        assert!(!lhs.matches(&rhs));

        // lhs is exactly the same as rhs, with no tolerance (so DOES match)
        let lhs = Needle::Timestamp(Timestamp::new(datetime!(2024-01-01 00:00:00)));
        let rhs = Needle::Timestamp(Timestamp::new(datetime!(2024-01-01 00:00:00)));

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
