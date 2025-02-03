pub mod variant;

use anyhow::Result;
use ipnet::Ipv4Net;
use std::net::Ipv4Addr;
use variant::IPv4Variant;

use super::{variant::NeedleVariant, Discombobulate, Matches};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IPv4 {
    pub value: Ipv4Addr,
    pub tolerance: Option<Ipv4Net>,
}

impl IPv4 {
    pub fn new(ipaddr: Ipv4Addr) -> Result<Self> {
        Ok(Self {
            value: ipaddr,
            tolerance: None,
        })
    }

    pub fn with_tolerance(ipaddr: Ipv4Addr, cidr_prefix: u8) -> Result<Self> {
        Ok(Self {
            value: ipaddr,
            tolerance: Some(Ipv4Net::new(ipaddr, cidr_prefix)?),
        })
    }
}

impl Matches for IPv4 {
    fn matches(&self, rhs: &Self) -> bool {
        // If rhs has a tolerance, check that lhs falls wthin it
        match &rhs.tolerance {
            Some(tolerance) => tolerance.contains(&self.value),
            None => self.value == rhs.value,
        }
    }
}

impl Discombobulate for IPv4 {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        let ip_as_u32 = self.value.to_bits();

        let needle_variants = ip_as_u32.discombobulate();

        for needle_variant in &needle_variants {
            if let NeedleVariant::Integer(v) = needle_variant {
                variants.push(NeedleVariant::IpAddr(IPv4Variant::Numeric(v.clone())));
            }
        }

        variants
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn cidr_test() {
        let actual = IPv4::new("192.168.0.1".parse().unwrap()).unwrap();
        let target = IPv4::with_tolerance("192.168.0.128".parse().unwrap(), 24).unwrap();

        assert!(actual.matches(&target));

        let actual = IPv4::new("192.168.1.2".parse().unwrap()).unwrap();
        let target = IPv4::with_tolerance("192.168.0.0".parse().unwrap(), 16).unwrap();

        assert!(actual.matches(&target));
    }

    #[test]
    fn discombobulation_test() {
        let actual = IPv4::new("192.168.0.1".parse().unwrap()).unwrap();

        let variants = actual.discombobulate();

        for variant in &variants {
            println!("{:?}", variant);
        }
    }
}
