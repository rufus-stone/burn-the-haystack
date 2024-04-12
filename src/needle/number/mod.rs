pub mod primitives;
pub mod variants;

use super::{variant::NeedleVariant, Discombobulate, Matches};

//#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Integer {
    pub value: i64,
    pub tolerance: Option<i64>,
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
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        // i64
        let mut i64_variants = self.value.discombobulate();
        variants.append(&mut i64_variants);

        // u64
        let mut u64_variants = (self.value as u64).discombobulate();
        variants.append(&mut u64_variants);

        // i32
        if (i32::MIN as i64..=i32::MAX as i64).contains(&self.value) {
            let mut i32_variants = (self.value as i32).discombobulate();
            variants.append(&mut i32_variants);
        }

        // u32
        if (u32::MIN as i64..=u32::MAX as i64).contains(&self.value) {
            let mut u32_variants = (self.value as u32).discombobulate();
            variants.append(&mut u32_variants);
        }

        // i16
        if (i16::MIN as i64..=i16::MAX as i64).contains(&self.value) {
            let mut i16_variants = (self.value as i16).discombobulate();
            variants.append(&mut i16_variants);
        }

        // u16
        if (u16::MIN as i64..=u16::MAX as i64).contains(&self.value) {
            let mut u16_variants = (self.value as u16).discombobulate();
            variants.append(&mut u16_variants);
        }

        // i8
        if (i8::MIN as i64..=i8::MAX as i64).contains(&self.value) {
            let mut i8_variants = (self.value as i8).discombobulate();
            variants.append(&mut i8_variants);
        }

        // u8
        if (u8::MIN as i64..=u8::MAX as i64).contains(&self.value) {
            let mut u8_variants = (self.value as u8).discombobulate();
            variants.append(&mut u8_variants);
        }

        variants
    }
}

#[derive(Clone, Debug, PartialOrd)]
pub struct Float {
    pub value: f64,
    pub tolerance: Option<f64>,
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        // Deliberately drop precision to f32 to allow for comparison between Floats created with an f64 and Floats recombobulated from an f32 variant
        self.value as f32 == other.value as f32 && self.tolerance == other.tolerance
        // TODO: impl PartialEq for all Needles and use .matches() instead of ==
    }
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
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

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
}
