use anyhow::Result;

use itertools::Itertools;
use macaddr::MacAddr6;
use oui_lookup::oui_db;

use super::{number::variants::IntegerVariant, variant::NeedleVariant, Discombobulate, Matches};

pub mod oui_lookup;
pub mod variant;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum MACTolerance {
    SameOUI,
    SameCompany,
    SpecificCompany(String),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MACAddr {
    value: Option<MacAddr6>,
    tolerance: Option<MACTolerance>,
}

impl MACAddr {
    pub fn new(macaddr: MacAddr6) -> Result<Self> {
        Ok(Self {
            value: Some(macaddr),
            tolerance: None,
        })
    }

    pub fn with_tolerance(macaddr: MacAddr6, tolerance: MACTolerance) -> Result<Self> {
        Ok(Self {
            value: Some(macaddr),
            tolerance: Some(tolerance),
        })
    }

    pub fn with_company(company: String) -> Result<Self> {
        Ok(Self {
            value: None,
            tolerance: Some(MACTolerance::SpecificCompany(company)),
        })
    }
}

impl Matches for MACAddr {
    fn matches(&self, rhs: &Self) -> bool {
        // If rhs has a tolerance, check that lhs falls wthin it
        match &rhs.tolerance {
            Some(tolerance) => match tolerance {
                MACTolerance::SameOUI => {
                    //self.value.as_bytes()[..3] == rhs.value.as_bytes()[..3],
                    if let Some(lhs_value) = self.value {
                        if let Some(rhs_value) = rhs.value {
                            return lhs_value.as_bytes()[..3] == rhs_value.as_bytes()[..3];
                        }
                    }
                    false
                }
                MACTolerance::SameCompany => {
                    if let Some(lhs_value) = self.value {
                        if let Some(rhs_value) = rhs.value {
                            // TODO: Avoid having to reload the entire database every time!
                            if let Ok(oui_db) = oui_db() {
                                //Oui::default() {
                                if let Ok(Some(lhs_info)) =
                                    oui_db.lookup_by_mac(&lhs_value.to_string())
                                {
                                    if let Ok(Some(rhs_info)) =
                                        oui_db.lookup_by_mac(&rhs_value.to_string())
                                    {
                                        // println!("lhs: {:?}", &lhs_info);
                                        // println!("rhs {:?}", &rhs_info);
                                        return lhs_info.company_name == rhs_info.company_name;
                                    }
                                }
                            }
                        }
                    }

                    false
                }
                MACTolerance::SpecificCompany(company_name) => {
                    if let Some(lhs_value) = self.value {
                        if let Ok(oui_db) = oui_db() {
                            // Oui::default() {
                            if let Ok(Some(lhs_info)) = oui_db.lookup_by_mac(&lhs_value.to_string())
                            {
                                // println!("lhs: {:?}", &lhs_info);
                                // println!("rhs {:?}", &company_name);
                                return &lhs_info.company_name == company_name;
                            }
                        }
                    }

                    false
                }
            },
            None => self.value == rhs.value,
        }
    }
}

impl Discombobulate for MACAddr {
    fn discombobulate(&self) -> Vec<NeedleVariant> {
        let mut variants = Vec::<NeedleVariant>::new();

        if let Some(value) = self.value {
            // u48 little endian
            let rev = value.as_bytes().iter().rev().cloned().collect_vec();
            if let Ok(v) = IntegerVariant::as_u48_le(&rev) {
                variants.push(NeedleVariant::Integer(v));
            }

            // u48 big endian
            if let Ok(v) = IntegerVariant::as_u48_be(value.as_bytes()) {
                variants.push(NeedleVariant::Integer(v));
            }
        }

        variants
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn mac_match_test() {
        // Exact match
        let actual = MACAddr::new("E0:8F:4C:11:22:33".parse().unwrap()).unwrap();
        let target = MACAddr::new("E0:8F:4C:11:22:33".parse().unwrap()).unwrap();

        assert!(actual.matches(&target));

        // Same OUI
        let actual = MACAddr::new("E0:8F:4C:11:22:33".parse().unwrap()).unwrap();
        let target =
            MACAddr::with_tolerance("E0:8F:4C:AA:BB:CC".parse().unwrap(), MACTolerance::SameOUI)
                .unwrap();

        assert!(actual.matches(&target));

        // Different OUI, same company (Google, Inc)
        let actual = MACAddr::new("D4:3A:2C:12:34:56".parse().unwrap()).unwrap();
        let target = MACAddr::with_tolerance(
            "54:60:09:AA:BB:CC".parse().unwrap(),
            MACTolerance::SameCompany,
        )
        .unwrap();

        assert!(actual.matches(&target));

        // Different OUI, same company (Intel Corp)
        let actual = MACAddr::new("E0:8F:4C:11:22:33".parse().unwrap()).unwrap();
        let target = MACAddr::with_tolerance(
            "80:32:53:AA:BB:CC".parse().unwrap(),
            MACTolerance::SameCompany,
        )
        .unwrap();

        assert!(actual.matches(&target));

        // Only company
        let actual = MACAddr::new("44:38:39:AA:BB:CC".parse().unwrap()).unwrap();
        let target = MACAddr::with_company("Cumulus Networks, Inc".to_owned()).unwrap();

        assert!(actual.matches(&target));

        // Not real MAC
        let actual = MACAddr::new("11:22:33:44:55:66".parse().unwrap()).unwrap();
        let target = MACAddr::with_company("Cumulus Networks, Inc".to_owned()).unwrap();

        assert!(!actual.matches(&target));

        // Not real company
        let actual = MACAddr::new("44:38:39:DD:EE:FF".parse().unwrap()).unwrap();
        let target = MACAddr::with_company("Made Up Corp, Inc".to_owned()).unwrap();

        assert!(!actual.matches(&target));
    }

    #[test]
    fn discombobulation_test() {
        let actual = MACAddr::new("11:22:33:44:55:66".parse().unwrap()).unwrap();

        let variants = actual.discombobulate();

        for variant in &variants {
            println!("{:?}", variant);
        }
    }
}
