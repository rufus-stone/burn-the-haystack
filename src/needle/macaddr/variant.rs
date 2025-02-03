use anyhow::{anyhow, Result};
use macaddr::MacAddr6;

use crate::needle::{number::variants::IntegerVariant, Interpret, Needle, Recombobulate};

use super::MACAddr;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MACAddrVariant {
    Numeric(IntegerVariant),
    // LE([u8; 6]),
    // BE([u8; 6]),
}

const MAC_ADDR_MAX: i64 = 281474976710655; // FF:FF:FF:FF:FF:FF

impl Recombobulate for MACAddrVariant {
    fn recombobulate(&self) -> Result<Needle> {
        match self {
            MACAddrVariant::Numeric(v) => {
                if let Ok(Needle::Integer(integer)) = v.recombobulate() {
                    if (0i64..=MAC_ADDR_MAX).contains(&integer.value) {
                        let a = ((integer.value as u64) >> 5) as u8;
                        let b = ((integer.value as u64) >> 4) as u8;
                        let c = ((integer.value as u64) >> 3) as u8;
                        let d = ((integer.value as u64) >> 2) as u8;
                        let e = ((integer.value as u64) >> 1) as u8;
                        let f = (integer.value as u64) as u8;
                        Ok(Needle::MacAddr(MACAddr::new(MacAddr6::new(
                            a, b, c, d, e, f,
                        ))?))
                    } else {
                        Err(anyhow!("Failed to recreate Needle::MacAddr from Integer"))
                    }
                } else {
                    Err(anyhow!("Failed to recreate Needle::MacAddr from Integer"))
                }
            } // MACAddrVariant::LE(v) => todo!(),
              // MACAddrVariant::BE(v) => todo!(),
        }
    }
}

impl Interpret for MACAddrVariant {
    fn interpret(data: &[u8]) -> Result<Vec<Self>>
    where
        Self: std::marker::Sized,
    {
        let mut intepretations = Vec::<Self>::new();

        // First, interpret as integers
        if let Ok(integer_variants) = IntegerVariant::interpret(data) {
            for variant in &integer_variants {
                // Only keep valid u48 variants
                if matches!(
                    variant,
                    IntegerVariant::U48LE((_, _)) | IntegerVariant::U48BE((_, _))
                ) {
                    intepretations.push(MACAddrVariant::Numeric(variant.clone()))
                }
            }
        }

        if intepretations.is_empty() {
            Err(anyhow!(
                "Failed to interpret bytes as any valid MACAddrVariant!"
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
        let actual_macaddr = MACAddr::new("AA:BB:CC:DD:EE:FF".parse().unwrap()).unwrap();

        println!("Actual             : {:02x?}", actual_macaddr);

        let variants = actual_macaddr.discombobulate();

        for variant in &variants {
            println!("Discombobulated as : {:02x?}", &variant);
            if let Ok(Needle::MacAddr(recom_macaddr)) = variant.recombobulate() {
                assert_eq!(actual_macaddr, recom_macaddr);
            }
        }
    }
}
