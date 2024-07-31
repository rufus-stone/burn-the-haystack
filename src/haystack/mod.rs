use itertools::Itertools;

use crate::needle::{
    ipaddr::variant::IPv4Variant,
    location::variant::LocationVariant,
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::variants::TimestampVariant,
    variant::NeedleVariant,
    Interpret, Matches, Needle, Recombobulate,
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

            // Location
            if let Ok(variants) = LocationVariant::interpret(window) {
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
                                    NeedleVariant::Location(variant.clone()),
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

            // IP Address
            if let Ok(variants) = IPv4Variant::interpret(window) {
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
                                    NeedleVariant::IpAddr(variant.clone()),
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
    use measurements::Distance;
    use time::Duration;

    use crate::needle::number::Integer;

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

        let y2k = Needle::new_timestamp("2000-01-01 00:00:00").unwrap();
        let nye23 = Needle::new_timestamp("2023-12-31 23:59:59").unwrap();
        let actual =
            Needle::new_timestamp_with_tolerance("2024-01-02 12:00:00", Duration::days(1)).unwrap();

        let needles = vec![y2k, nye23, actual];

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

        let y2k = Needle::new_timestamp("2000-01-01 00:00:00").unwrap();
        let nye23 = Needle::new_timestamp("2023-12-31 23:59:59").unwrap();
        let actual =
            Needle::new_timestamp_with_tolerance("2024-01-02 12:00:00", Duration::days(1)).unwrap();

        let needles = vec![y2k, nye23, actual];

        let haystack = Haystack::with_needles(data, needles.clone());

        let results = haystack.burn();

        for result in &results {
            println!("{:?}", result);
        }

        assert!(results.len() == 1); // There should be only one match
        assert!(results[0].actual.matches(&needles[2])); // It should have matched on the "actual" timestamp
        assert!(matches!(
            results[0].variant,
            NeedleVariant::Timestamp(TimestampVariant::DOSTime(IntegerVariant::U32LE(_)))
        )) // And the variant that matched should have been a DOSTime built using an unsigned 32bit little endian integer
    }

    #[test]
    fn location_needles_test() {
        // Some random bytes with an set of coordinates in the middle: -31.95, 115.85 DecimalMinutesLatLon(F32LE)
        let data: Vec<u8> = vec![
            0xde, 0xad, 0xbe, 0xef, 0x00, 0xa0, 0xef, 0xc4, 0x00, 0x38, 0xd9, 0x45, 0xca, 0xfe,
            0xba, 0xbe,
        ];

        let nyc =
            Needle::new_location_with_tolerance(40.73, -74.03, Distance::from_kilometres(100.0))
                .unwrap(); // 40.73, -74.03
        let perth =
            Needle::new_location_with_tolerance(-31.9525, 115.8500, Distance::from_kilometres(5.0))
                .unwrap(); // -31.9525, 115.8500

        let needles = vec![nyc, perth];

        let haystack = Haystack::with_needles(data, needles.clone());

        let results = haystack.burn();

        for result in &results {
            println!("{:02x?}", result);
        }

        assert!(results.len() == 1); // There should be only one match
        assert!(results[0].actual.matches(&needles[1])); // It should have matched on the "perth" location
        assert!(matches!(
            results[0].variant,
            NeedleVariant::Location(LocationVariant::DecimalMinutesLatLon(
                FloatVariant::F32LE(_),
                FloatVariant::F32LE(_)
            ))
        )) // And the variant that matched should have been a DecimalMinutesLatLon built using two 32bit little endian floats
    }

    #[test]
    fn complex_test() {
        // Some random bytes with an set of coordinates in the middle (-31.95, 115.85 DecimalMinutesLatLon(F32LE)), a timestamp (2023-12-31 23:59:58 EpochNanos(I64Varint)), and an IP address (192.168.0.1)
        let data: Vec<u8> = vec![
            0xde, 0xad, 0xbe, 0xef, 0x00, 0xa0, 0xef, 0xc4, 0x00, 0x38, 0xd9, 0x45, 0xca, 0xfe,
            0xba, 0xbe, 0x80, 0xb0, 0xfb, 0xa2, 0xd1, 0x85, 0x88, 0xa6, 0x2f, 0x00, 0x00, 0x00,
            0xc0, 0xa8, 0x00, 0x01, 0xff, 0xff,
        ];

        // Locations
        let nyc =
            Needle::new_location_with_tolerance(40.73, -74.03, Distance::from_kilometres(100.0))
                .unwrap(); // 40.73, -74.03
        let perth =
            Needle::new_location_with_tolerance(-31.9525, 115.8500, Distance::from_kilometres(5.0))
                .unwrap(); // -31.9525, 115.8500

        // Timestamps
        let mid_december =
            Needle::new_timestamp_with_tolerance("2023-12-15 00:00:00", Duration::days(30))
                .unwrap();
        let august =
            Needle::new_timestamp_with_tolerance("2023-08-10 00:00:00", Duration::days(60))
                .unwrap();

        // IPs
        let google_dns = Needle::new_ip_address("8.8.8.8".parse().unwrap()).unwrap();
        let private_ip =
            Needle::new_ip_address_with_tolerance("192.168.0.0".parse().unwrap(), 16).unwrap();

        // Configure the needles and haystack
        let needles = vec![nyc, perth, mid_december, august, google_dns, private_ip];

        let haystack = Haystack::with_needles(data, needles.clone());

        // Burn the haystack
        let results = haystack.burn();

        for result in &results {
            println!("{:02x?}", result);
        }

        assert!(results.len() == 3); // There should be 3 matches

        assert!(results[0].actual.matches(&needles[1])); // It should have matched on the "perth" location
        assert!(results[1].actual.matches(&needles[2])); // It should have matched on the "mid_december" timestamp
        assert!(results[2].actual.matches(&needles[5])); // It should have matched on the "private_ip" ip address

        assert!(matches!(
            results[0].variant,
            NeedleVariant::Location(LocationVariant::DecimalMinutesLatLon(
                FloatVariant::F32LE(_),
                FloatVariant::F32LE(_)
            ))
        )); // The location variant that matched should have been a DecimalMinutesLatLon built using two 32bit little endian floats

        assert!(matches!(
            results[1].variant,
            NeedleVariant::Timestamp(TimestampVariant::EpochNanos(IntegerVariant::I64Varint(_)))
        )); // The timestamp variant that matched should have been an EpochNanos built using an I64Varint

        assert!(matches!(
            results[2].variant,
            NeedleVariant::IpAddr(IPv4Variant::Numeric(IntegerVariant::U32BE(_)))
        )); // The IP address variant that matched should have been built using a U32BE integer
    }
}
