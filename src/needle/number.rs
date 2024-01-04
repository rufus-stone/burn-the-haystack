use integer_encoding::VarInt;

use super::Discombobulate;

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
