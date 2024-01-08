use ipnet::Ipv4Net;
use std::net::Ipv4Addr;

#[derive(Clone, Debug, PartialEq)]
pub struct IPv4 {
    pub value: Ipv4Addr,
    pub tolerance: Option<u8>,
}
