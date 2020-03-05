use std::fmt;
use std::fmt::{Error, Formatter};
use std::net::Ipv4Addr;

use ipnetwork::IpNetwork;
use pnet::datalink::{self, NetworkInterface};


#[derive(Clone, Eq)]
pub struct MacAddr(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);
#[derive(Copy, Clone)]
pub struct IpAddr(pub u8, pub u8, pub u8, pub u8);

impl PartialEq for MacAddr {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
            && self.1 == other.1
            && self.2 == other.2
            && self.3 == other.3
            && self.4 == other.4
            && self.5 == other.5
    }
}

impl From<Ipv4Addr> for IpAddr {
    fn from(ip: Ipv4Addr) -> Self {
        let octs = ip.octets();
        return IpAddr(octs[0], octs[1], octs[2], octs[3]);
    }
}

impl fmt::Display for MacAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let beauty_mac = format!(
            "{:0>2x}:{:0>2x}:{:0>2x}:{:0>2x}:{:0>2x}:{:0>2x}",
            self.0, self.1, self.2, self.3, self.4, self.5
        );
        write!(f, "{}", beauty_mac)
    }
}

impl MacAddr {
    pub fn local() -> Self {
        match default_interface().mac_address() {
            pnet::datalink::MacAddr(a, b, c, d, e, f) => MacAddr(a, b, c, d, e, f),
        }
    }
}


impl IpAddr {
    pub fn local() -> Self {
        let sender_ip = default_interface()
            .ips
            .iter()
            .filter(|&network| match network {
                IpNetwork::V4(_) => true,
                _ => false,
            })
            .map(|&network| match network {
                IpNetwork::V4(addr) => IpAddr::from(addr.ip()),
                _ => IpAddr(0, 0, 0, 0),
            })
            .next()
            .unwrap();
        sender_ip
    }
}

pub fn default_interface() -> NetworkInterface {
    let interfaces = datalink::interfaces();
    let interface_names_match = |iface: &NetworkInterface| &iface.mac != &Some(pnet::datalink::MacAddr::zero());
    let iface = interfaces
        .into_iter()
        .filter(interface_names_match)
        .next()
        .unwrap();
    iface
}


impl fmt::Display for IpAddr {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let beauty_ip = format!("{}.{}.{}.{}", self.0, self.1, self.2, self.3);
        write!(f, "{}", beauty_ip)
    }
}

pub trait Serializable {
    fn to_hex(&self) -> Vec<u8>;
}

impl Serializable for MacAddr {
    fn to_hex(&self) -> Vec<u8> {
        return vec![self.0, self.1, self.2, self.3, self.4, self.5];
    }
}

impl Serializable for IpAddr {
    fn to_hex(&self) -> Vec<u8> {
        vec![self.0, self.1, self.2, self.3]
    }
}
