pub mod number;
pub mod timestamp;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Needle {
    Timestamp(timestamp::Timestamp),
    Location,
    IpAddr,
    MacAddr,
    Integer(i64),
    Float(f64),
    Bytes(Vec<u8>),
}

impl Needle {
    pub fn matches(&self, other: &[u8]) -> bool {
        match &self {
            Needle::Timestamp(lhs) => {
                //
                todo!()
            }
            Needle::Location => todo!(),
            Needle::IpAddr => todo!(),
            Needle::MacAddr => todo!(),
            Needle::Integer(_) => todo!(),
            Needle::Float(_) => todo!(),
            Needle::Bytes(_) => todo!(),
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
            Needle::Location => todo!(),
            Needle::IpAddr => todo!(),
            Needle::MacAddr => todo!(),
            Needle::Integer(i) => {
                let mut variants = Vec::<(Vec<u8>, String)>::new();

                // i64
                let mut i64_variants = i.discombobulate();
                variants.append(&mut i64_variants);

                // u64
                let mut u64_variants = (*i as u64).discombobulate();
                variants.append(&mut u64_variants);

                // i32
                if *i < (i32::MAX as i64) && *i > (i32::MIN as i64) {
                    let mut i32_variants = (*i as i32).discombobulate();
                    variants.append(&mut i32_variants);
                }

                // u32
                if *i < (u32::MAX as i64) && *i > (u32::MIN as i64) {
                    let mut u32_variants = (*i as u32).discombobulate();
                    variants.append(&mut u32_variants);
                }

                // i16
                if *i < (i16::MAX as i64) && *i > (i16::MIN as i64) {
                    let mut i16_variants = (*i as i16).discombobulate();
                    variants.append(&mut i16_variants);
                }

                // u16
                if *i < (u16::MAX as i64) && *i > (u16::MIN as i64) {
                    let mut u16_variants = (*i as u16).discombobulate();
                    variants.append(&mut u16_variants);
                }

                // i8
                if *i < (i8::MAX as i64) && *i > (i8::MIN as i64) {
                    let mut i8_variants = (*i as i8).discombobulate();
                    variants.append(&mut i8_variants);
                }

                // u8
                if *i < (u8::MAX as i64) && *i > (u8::MIN as i64) {
                    let mut u8_variants = (*i as u8).discombobulate();
                    variants.append(&mut u8_variants);
                }

                variants
            }
            Needle::Float(f) => {
                let mut variants = Vec::<(Vec<u8>, String)>::new();

                // f64
                let mut f64_variants = f.discombobulate();
                variants.append(&mut f64_variants);

                // f32
                if *f < (f32::MAX as f64) && *f > (f32::MIN as f64) {
                    let mut f32_variants = (*f as f32).discombobulate();
                    variants.append(&mut f32_variants);
                }

                variants
            }
            Needle::Bytes(bytes) => vec![(bytes.to_vec(), String::from("Byte sequence"))],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    #[test]
    fn integer_zero() {
        let needle = Needle::Integer(0);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_negative() {
        let needle = Needle::Integer(-3);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn integer_positive() {
        let needle = Needle::Integer(12345);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_zero() {
        let needle = Needle::Float(0.0);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_negative() {
        let needle = Needle::Float(-1.0);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }

    #[test]
    fn float_positive() {
        let needle = Needle::Float(2.2);
        let variants = needle.discombobulate();

        println!("{:?} ->", needle);

        for variant in variants {
            println!("{:>20} : {:02x?}", variant.1, variant.0);
        }

        assert_eq!(1, 1);
    }
}
