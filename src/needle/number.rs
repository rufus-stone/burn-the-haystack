use integer_encoding::VarInt;

use super::{Discombobulate, Matches};

//#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone, Debug, PartialEq)]
pub struct Integer {
    value: i64,
    tolerance: Option<i64>,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            tolerance: None,
        }
    }

    pub fn with_tolerance(value: i64, tolerance: i64) -> Self {
        Self {
            value,
            tolerance: Some(tolerance),
        }
    }
}

impl Matches for Integer {
    fn matches(&self, rhs: &Self) -> bool {
        // If rhs has a tolerance, check that lhs falls wthin it
        match rhs.tolerance {
            Some(tolerance) => {
                let actual_difference = (self.value - rhs.value).abs();
                let max_allowed_difference = tolerance.abs();

                // println!("Actual dif: {}", actual_difference);
                // println!("Allowed dif: {}", max_allowed_difference);

                actual_difference <= max_allowed_difference
            }
            None => self.value == rhs.value,
        }
    }
}

impl Discombobulate for Integer {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        // i64
        let mut i64_variants = self.value.discombobulate();
        variants.append(&mut i64_variants);

        // u64
        let mut u64_variants = (self.value as u64).discombobulate();
        variants.append(&mut u64_variants);

        // i32
        if (i32::MIN as i64..=i32::MAX as i64).contains(&self.value) {
            //if self.value < (i32::MAX as i64) && self.value > (i32::MIN as i64) {
            let mut i32_variants = (self.value as i32).discombobulate();
            variants.append(&mut i32_variants);
        }

        // u32
        if (u32::MIN as i64..=u32::MAX as i64).contains(&self.value) {
            //if self.value < (u32::MAX as i64) && self.value > (u32::MIN as i64) {
            let mut u32_variants = (self.value as u32).discombobulate();
            variants.append(&mut u32_variants);
        }

        // i16
        if (i16::MIN as i64..=i16::MAX as i64).contains(&self.value) {
            //if self.value < (i16::MAX as i64) && self.value > (i16::MIN as i64) {
            let mut i16_variants = (self.value as i16).discombobulate();
            variants.append(&mut i16_variants);
        }

        // u16
        if (u16::MIN as i64..=u16::MAX as i64).contains(&self.value) {
            //if self.value < (u16::MAX as i64) && self.value > (u16::MIN as i64) {
            let mut u16_variants = (self.value as u16).discombobulate();
            variants.append(&mut u16_variants);
        }

        // i8
        if (i8::MIN as i64..=i8::MAX as i64).contains(&self.value) {
            //if self.value < (i8::MAX as i64) && self.value > (i8::MIN as i64) {
            let mut i8_variants = (self.value as i8).discombobulate();
            variants.append(&mut i8_variants);
        }

        // u8
        if (u8::MIN as i64..=u8::MAX as i64).contains(&self.value) {
            //self.value < (u8::MAX as i64) && self.value > (u8::MIN as i64) {
            let mut u8_variants = (self.value as u8).discombobulate();
            variants.append(&mut u8_variants);
        }

        variants
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Float {
    value: f64,
    tolerance: Option<f64>,
}

impl Float {
    pub fn new(value: f64) -> Self {
        Self {
            value,
            tolerance: None,
        }
    }

    pub fn with_tolerance(value: f64, tolerance: f64) -> Self {
        Self {
            value,
            tolerance: Some(tolerance),
        }
    }
}

impl Matches for Float {
    fn matches(&self, rhs: &Self) -> bool {
        // If rhs has a tolerance, check that lhs falls wthin it
        match rhs.tolerance {
            Some(tolerance) => {
                let actual_difference = (self.value - rhs.value).abs();
                let max_allowed_difference = tolerance.abs();

                println!("Actual dif: {}", actual_difference);
                println!("Allowed dif: {}", max_allowed_difference);

                actual_difference <= max_allowed_difference
            }
            None => self.value == rhs.value,
        }
    }
}

impl Discombobulate for Float {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        // f64
        let mut f64_variants = self.value.discombobulate();
        variants.append(&mut f64_variants);

        // f32
        if (f32::MIN as f64..=f32::MAX as f64).contains(&self.value) {
            //if self.value < (f32::MAX as f64) && self.value > (f32::MIN as f64) {
            let mut f32_variants = (self.value as f32).discombobulate();
            variants.append(&mut f32_variants);
        }

        variants
    }
}

// Basic numerics

impl Discombobulate for u8 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("u8")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("u8 varint")));

        variants
    }
}

impl Discombobulate for i8 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("i8")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("i8 varint")));

        variants
    }
}

impl Discombobulate for u16 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("u16 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("u16 big endian")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("u16 varint")));

        variants
    }
}

impl Discombobulate for i16 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("i16 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("i16 big endian")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("i16 varint")));

        variants
    }
}

impl Discombobulate for u32 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("u32 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("u32 big endian")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("u32 varint")));

        variants
    }
}

impl Discombobulate for i32 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("i32 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("i32 big endian")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("i32 varint")));

        variants
    }
}

impl Discombobulate for u64 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("u64 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("u64 big endian")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("u64 varint")));

        variants
    }
}

impl Discombobulate for i64 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("i64 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("i64 big endian")));

        let varint = self.encode_var_vec();
        variants.push((varint, String::from("i64 varint")));

        variants
    }
}

impl Discombobulate for f32 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("f32 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("f32 big endian")));

        variants
    }
}

impl Discombobulate for f64 {
    fn discombobulate(&self) -> Vec<(Vec<u8>, String)> {
        let mut variants = Vec::<(Vec<u8>, String)>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push((le_bytes, String::from("f64 little endian")));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push((be_bytes, String::from("f64 big endian")));

        variants
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn u8_test() {
        let n: u8 = 240;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn i8_test() {
        let n: i8 = -2;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn u16_test() {
        let n: u16 = 1234;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn i16_test() {
        let n: i16 = -1000;
        let variants = n.discombobulate();

        println!("{} -> {:02x?}", n, variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn u32_test() {
        let n: u32 = 240;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn i32_test() {
        let n: i32 = -12345;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn u64_test() {
        let n: u64 = 240;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn i64_test() {
        let n: i64 = 240;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn f32_test() {
        let n: f32 = std::f32::consts::PI;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }

    #[test]
    fn f64_test() {
        let n: f64 = std::f64::consts::PI;
        let variants = n.discombobulate();

        println!("{:02x?}", variants);

        assert_eq!(1, 1);
    }
}
