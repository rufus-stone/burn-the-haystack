use anyhow::{anyhow, Result};

use crate::needle::{number::variants::IntegerVariant, Interpret, Needle, Recombobulate};

use super::IPv4;

#[derive(Clone, Debug, PartialEq)]
pub enum IPv4Variant {
    Numeric(IntegerVariant),
}

impl Recombobulate for IPv4Variant {
    fn recombobulate(&self) -> anyhow::Result<crate::needle::Needle> {
        match self {
            IPv4Variant::Numeric(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    if (u32::MIN as i64..=u32::MAX as i64).contains(&integer.value) {
                        Ok(Needle::IpAddr(IPv4::new((integer.value as u32).into())?))
                    } else {
                        Err(anyhow!("Failed to recreate Needle::IpAddr from Integer"))
                    }
                } else {
                    Err(anyhow!("Failed to recreate Needle::IpAddr from Integer"))
                }
            }
        }
    }
}

impl Interpret for IPv4Variant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        // First, interpret as integers
        if let Ok(integer_variants) = IntegerVariant::interpret(data) {
            for variant in &integer_variants {
                // Only keep valid u32 variants
                if matches!(
                    variant,
                    IntegerVariant::U32LE((_, _))
                        | IntegerVariant::U32BE((_, _))
                        | IntegerVariant::U32Varint((_, _))
                ) {
                    intepretations.push(IPv4Variant::Numeric(variant.clone()))
                }
            }
        }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid IPv4Variant!"
            ))
        } else {
            Ok(intepretations)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::needle::Discombobulate;

    use super::*;

    #[test]
    fn recombobulation_test() {
        let actual_ipv4 = IPv4::new("192.168.0.1".parse().unwrap()).unwrap();

        //println!("Actual             : {:?}", actual_ipv4);

        let variants = actual_ipv4.discombobulate();

        for variant in &variants {
            //println!("Discombobulated as : {:02x?}", &variant);
            if let Ok(Needle::IpAddr(recom_ipv4)) = variant.recombobulate() {
                assert_eq!(actual_ipv4, recom_ipv4);
            }
        }
    }
}
