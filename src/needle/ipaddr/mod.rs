pub mod variant;

use ipnet::Ipv4Net;
use std::net::Ipv4Addr;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IPv4 {
    pub value: Ipv4Addr,
    pub tolerance: Option<u8>,
}
