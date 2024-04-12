use itertools::Itertools;

use crate::needle::{
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::variants::TimestampVariant,
    variant::NeedleVariant,
    Discombobulate, Interpret, Matches, Needle, Recombobulate,
};

use self::ashes::Ashes;

pub mod ashes;

pub struct Haystack {
    pub data: Vec<u8>,
    pub needles: Vec<Needle>,
}

impl Haystack {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
            needles: Default::default(),
        }
    }

    pub fn with_needles(data: Vec<u8>, needles: Vec<Needle>) -> Self {
        Self { data, needles }
    }

    pub fn burn(&self) -> Vec<Ashes> {
        let mut ash_pile = Vec::<Ashes>::new();

        // TODO - should we just auto-discombobulate all needles on creation?
        // let target_needle_variants = self
        //     .needles
        //     .iter()
        //     .flat_map(|needle| needle.discombobulate())
        //     .collect_vec();

        for i in 0..self.data.len() {
            let window = &self.data.as_slice()[i..];
            //println!("Window: {:02x?}", &self.data.as_slice()[i..]);

            // Try to interpret the bytes as all known variants

            // Integer
            if let Ok(variants) = IntegerVariant::interpret(window) {
                for variant in &variants {
                    //println!("{:?}", &variant);

                    if let Ok(putative) = variant.recombobulate() {
                        //println!("{:?}", &needle);

                        let hits = self
                            .needles
                            .iter()
                            .filter(|target| putative.matches(target))
                            .map(|target| {
                                //println!("It's a match!");
                                Ashes::new(
                                    target,
                                    putative.clone(),
                                    NeedleVariant::Integer(variant.clone()),
                                    i,
                                )
                            })
                            .collect_vec();

                        for hit in hits {
                            ash_pile.push(hit);
                        }
                    }
                }
            }

            // Float
            if let Ok(variants) = FloatVariant::interpret(window) {
                for variant in &variants {
                    //println!("{:?}", &variant);

                    if let Ok(putative) = variant.recombobulate() {
                        //println!("{:?}", &needle);

                        let hits = self
                            .needles
                            .iter()
                            .filter(|target| putative.matches(target))
                            .map(|target| {
                                //println!("It's a match!");
                                Ashes::new(
                                    target,
                                    putative.clone(),
                                    NeedleVariant::Float(variant.clone()),
                                    i,
                                )
                            })
                            .collect_vec();

                        for hit in hits {
                            ash_pile.push(hit);
                        }
                    }
                }
            }

            // Timestamp
            if let Ok(variants) = TimestampVariant::interpret(window) {
                for variant in &variants {
                    //println!("{:?}", &variant);

                    if let Ok(putative) = variant.recombobulate() {
                        //println!("{:?}", &needle);

                        let hits = self
                            .needles
                            .iter()
                            .filter(|target| putative.matches(target))
                            .map(|target| {
                                //println!("It's a match!");
                                Ashes::new(
                                    target,
                                    putative.clone(),
                                    NeedleVariant::Timestamp(variant.clone()),
                                    i,
                                )
                            })
                            .collect_vec();

                        for hit in hits {
                            ash_pile.push(hit);
                        }
                    }
                }
            }
        }

        ash_pile
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::datetime, Duration};

    use crate::needle::{number::Integer, timestamp::Timestamp};

    use super::*;

    #[test]
    fn new_haystack() {
        let data: Vec<u8> = vec![0x00];
        let haystack = Haystack::new(data);

        assert_eq!(haystack.data.len(), 1);
        assert!(haystack.needles.is_empty());
    }

    #[test]
    fn integer_needles_test() {
        // First 16 bytes of a ZIP file which contains a DOS timestamp
        let data: Vec<u8> = vec![
            0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x08, 0x00, 0x08, 0x00, 0x8e, 0x72, 0x22, 0x58,
            0x00, 0x00,
        ];

        let n1 = Integer::new(12345);
        let n2 = Integer::with_tolerance(-100, 3);
        let actual = Integer::with_tolerance(2389844560, 8);

        let needles = vec![
            Needle::Integer(n1),
            Needle::Integer(n2),
            Needle::Integer(actual),
        ];

        let haystack = Haystack::with_needles(data, needles);

        let ashes = haystack.burn();

        for ash in &ashes {
            println!("[ Target needle found ]");
            println!("Target  : {:?}", ash.target);
            println!("Actual  : {:?}", ash.actual);
            println!("Variant : {:02x?}", ash.variant);
            println!("Offset  : {}", ash.offset);
        }
    }

    #[test]
    fn timestamp_needles_test() {
        // First 16 bytes of a ZIP file which contains a DOS timestamp
        let data: Vec<u8> = vec![
            0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x08, 0x00, 0x08, 0x00, 0x8e, 0x72, 0x22, 0x58,
            0x00, 0x00,
        ];

        let y2k = Timestamp::new(datetime!(2000-01-01 00:00:00));
        let nye23 = Timestamp::new(datetime!(2023-12-31 23:59:59));
        let actual = Timestamp::with_tolerance(datetime!(2024-01-02 12:00:00), Duration::days(1));

        let needles = vec![
            Needle::Timestamp(y2k),
            Needle::Timestamp(nye23),
            Needle::Timestamp(actual),
        ];

        let haystack = Haystack::with_needles(data, needles);

        let ashes = haystack.burn();

        for ash in &ashes {
            println!("[ Target needle found ]");
            println!("Target  : {:?}", ash.target);
            println!("Actual  : {:?}", ash.actual);
            println!("Variant : {:02x?}", ash.variant);
            println!("Offset  : {}", ash.offset);
        }
    }

    #[test]
    fn timestamp_needles_test2() {
        // First 16 bytes of a ZIP file which contains a DOS timestamp
        let data: Vec<u8> = vec![
            0x50, 0x4b, 0x03, 0x04, 0x14, 0x00, 0x08, 0x00, 0x08, 0x00, 0x8e, 0x72, 0x22, 0x58,
            0x00, 0x00,
        ];

        let y2k = Timestamp::new(datetime!(2000-01-01 00:00:00));
        let nye23 = Timestamp::new(datetime!(2023-12-31 23:59:59));
        let actual = Timestamp::with_tolerance(datetime!(2024-01-02 12:00:00), Duration::days(1));

        let needles = vec![
            Needle::Timestamp(y2k),
            Needle::Timestamp(nye23),
            Needle::Timestamp(actual),
        ];

        let haystack = Haystack::with_needles(data, needles);

        let results = haystack.burn();

        for result in &results {
            println!("{:?}", result);
        }
    }
}
