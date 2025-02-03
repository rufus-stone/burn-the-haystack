use itertools::Itertools;

use crate::needle::{
    ipaddr::variant::IPv4Variant,
    location::variant::LocationVariant,
    number::variants::{FloatVariant, IntegerVariant},
    timestamp::variants::TimestampVariant,
    variant::NeedleVariant,
    Interpret, Matches, Needle, Recombobulate,
};

use super::ashes::Ashes;

pub struct Hay<'a> {
    datas: Vec<HayData<'a>>,
    needles: Vec<Needle>,
}

impl<'a> Hay<'a> {
    pub fn new(datas: Vec<HayData<'a>>) -> Self {
        Self {
            datas,
            needles: Default::default(),
        }
    }

    pub fn with_needles(datas: Vec<HayData<'a>>, needles: Vec<Needle>) -> Self {
        Self { datas, needles }
    }

    pub fn burn(&self) -> Vec<Ashes> {
        let mut ash_pile = Vec::<Ashes>::new();

        for data in &self.datas {
            for i in 0..data.len() {
                let window = &data.as_slice()[i..];
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
        }

        ash_pile
    }
}

pub enum HayData<'a> {
    Capture(etherparse::SlicedPacket<'a>),
    Raw(Vec<u8>),
}

impl<'a> HayData<'a> {
    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn is_empty(&self) -> bool {
        self.as_slice().is_empty()
    }

    pub fn as_slice(&'a self) -> &'a [u8] {
        match &self {
            HayData::Capture(cap) => {
                if let Some(transport) = &cap.transport {
                    match transport {
                        etherparse::TransportSlice::Icmpv4(icmpv4) => icmpv4.payload(),
                        etherparse::TransportSlice::Icmpv6(icmpv6) => icmpv6.payload(),
                        etherparse::TransportSlice::Udp(udp) => udp.payload(),
                        etherparse::TransportSlice::Tcp(tcp) => tcp.payload(),
                    }
                } else if let Some(net) = &cap.net {
                    match net {
                        etherparse::NetSlice::Ipv4(ipv4) => ipv4.payload().payload,
                        etherparse::NetSlice::Ipv6(ipv6) => ipv6.payload().payload,
                    }
                } else if let Some(link) = &cap.link {
                    match link {
                        etherparse::LinkSlice::Ethernet2(eth2) => eth2.payload_slice(),
                        etherparse::LinkSlice::LinuxSll(sll) => sll.payload_slice(),
                        etherparse::LinkSlice::EtherPayload(eth) => eth.payload,
                        etherparse::LinkSlice::LinuxSllPayload(sll) => sll.payload,
                    }
                } else {
                    self.as_slice()
                }
            }
            HayData::Raw(raw) => raw.as_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use measurements::Distance;
    use time::Duration;

    use super::*;

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

        let hay = Hay::with_needles(vec![HayData::Raw(data)], needles.clone());

        // Burn the haystack
        let results = hay.burn();

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
