use itertools::Itertools;

use crate::needle::{Discombobulate, Needle};

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

        for needle in &self.needles {
            let needlets = needle.discombobulate();

            for (byte_sequence, name) in needlets {
                // TODO: Need to convert the window under investigation into the appropriate Needle type for comparison

                let mut hits = self
                    .data
                    .as_slice()
                    .windows(byte_sequence.len())
                    .positions(|window| window == byte_sequence) // TODO: Change this to fuzzy match based on needle tolerances
                    .map(|p| Ashes::new(needle, p, name.clone(), byte_sequence.clone()))
                    .collect_vec();

                ash_pile.append(&mut hits);
            }
        }

        ash_pile
    }
}

#[cfg(test)]
mod tests {
    use time::{macros::datetime, Duration};

    use crate::needle::timestamp::Timestamp;

    use super::*;

    #[test]
    fn new_haystack() {
        let data: Vec<u8> = vec![0x00];
        let haystack = Haystack::new(data);

        assert_eq!(haystack.data.len(), 1);
        assert!(haystack.needles.is_empty());
    }

    #[test]
    fn bytes_needles_test() {
        let data: Vec<u8> = vec![
            0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0x01, 0x02, 0x03, 0x04, 0xca, 0xfe,
            0xca, 0xfe, 0xba, 0xbe, 0xc0, 0x01, 0xd0, 0x0d,
        ];

        let cafe: &[u8] = &[0xca, 0xfe];
        let cafebabe: &[u8] = &[0xca, 0xfe, 0xba, 0xbe];
        let notreal: &[u8] = &[0xaa, 0xbb, 0xcc, 0xdd];

        let needles = vec![
            Needle::Bytes(cafe.to_vec()),
            Needle::Bytes(cafebabe.to_vec()),
            Needle::Bytes(notreal.to_vec()),
        ];

        let haystack = Haystack::with_needles(data, needles);

        let ash_pile = haystack.burn();

        println!("Burnt the haystack");

        println!("{:02x?}", ash_pile);
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

        let results = haystack.burn();
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
    }
}
