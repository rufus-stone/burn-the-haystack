use anyhow::Result;
use integer_encoding::VarInt;

use crate::needle::Recombobulate;
use crate::needle::{variant::NeedleVariant, Discombobulate};

use super::variants::FloatVariant::*;
use super::variants::IntegerVariant::*;

impl Discombobulate for u8 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U8((le_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(U8Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for i8 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I8((le_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(I8Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for u16 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U16LE((le_bytes, *self))));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U16BE((be_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(U16Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for i16 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I16LE((le_bytes, *self))));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I16BE((be_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(I16Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for u32 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U32LE((le_bytes, *self))));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U32BE((be_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(U32Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for i32 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I32LE((le_bytes, *self))));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I32BE((be_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(I32Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for u64 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U64LE((le_bytes, *self))));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(U64BE((be_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(U64Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for i64 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I64LE((le_bytes, *self))));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Integer(I64BE((be_bytes, *self))));

        let varint = self.encode_var_vec();
        variants.push(NeedleVariant::Integer(I64Varint((varint, *self))));

        variants
    }
}

impl Discombobulate for f32 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Float(F32LE(le_bytes)));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Float(F32BE(be_bytes)));

        variants
    }
}

impl Discombobulate for f64 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let le_bytes = self.to_le_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Float(F64LE(le_bytes)));

        let be_bytes = self.to_be_bytes().as_slice().to_owned();
        variants.push(NeedleVariant::Float(F64BE(be_bytes)));

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
