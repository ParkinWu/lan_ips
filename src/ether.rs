use crate::arp;
use crate::net::{MacAddr, Serializable};
use crate::parser::{ParseError, Parser};
use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub enum Protocol {
    ARP,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ARP")
    }
}

pub struct Packet {
    pub dest_addr: MacAddr,
    pub src_addr: MacAddr,
    pub protocol: Protocol,
    pub data: Data,
}

pub enum Data {
    ARP(arp::Packet),
    None,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "\n===============Ether Packet================\n");
        write!(f, "dest mac:    {}\n", self.dest_addr);
        write!(f, "src mac:     {}\n", self.src_addr);
        write!(f, "protocol:    {}\n", self.protocol);
        let data = match self.data {
            Data::ARP(ref packet) => {
                write!(f, "===============ARP Packet==================\n");
                format!("{}", packet)
            }
            Data::None => "no data".to_string(),
        };

        write!(f, "{}", data);
        write!(f, "===========================================\n");
        Ok(())
    }
}

impl<'a> Parser<'a> {
    fn parse_protocol(&mut self) -> Result<Protocol, ParseError> {
        let _ = self.expect(0x08)?;
        let _ = self.expect(0x06)?;
        Ok(Protocol::ARP)
    }

    fn parse_data(&mut self, protocol: &Protocol) -> Result<Data, ParseError> {
        let data = match protocol {
            Protocol::ARP => {
                let packet = arp::Packet::parse(&self.rest())?;
                Data::ARP(packet)
            }
        };
        Ok(data)
    }
}

impl Packet {
    pub fn parse(packet: &Vec<u8>) -> Result<Packet, ParseError> {
        let mut parser = Parser::new(packet);
        let dest_mac = parser.parse_mac()?;
        let src_mac = parser.parse_mac()?;
        let protocol = parser.parse_protocol()?;
        let data = parser.parse_data(&protocol)?;
        let packet = Packet {
            dest_addr: dest_mac,
            src_addr: src_mac,
            protocol,
            data,
        };
        Ok(packet)
    }

    pub fn new() -> Self {
        Packet {
            dest_addr: MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff),
            src_addr: MacAddr(0, 0, 0, 0, 0, 0),
            protocol: Protocol::ARP,
            data: Data::None,
        }
    }

    pub fn set_packet_content(&mut self, data: Data) {
        self.data = data;
    }

    pub fn set_dest_addr(&mut self, mac: MacAddr) {
        self.dest_addr = mac;
    }

    pub fn set_src_addr(&mut self, mac: MacAddr) {
        self.src_addr = mac;
    }
}

impl Serializable for Protocol {
    fn to_hex(&self) -> Vec<u8> {
        match self {
            Protocol::ARP => vec![0x08, 0x06],
        }
    }
}

impl Serializable for Packet {
    fn to_hex(&self) -> Vec<u8> {
        let mut packet = vec![];
        packet.extend(self.dest_addr.to_hex());
        packet.extend(self.src_addr.to_hex());
        packet.extend(self.protocol.to_hex());
        let data = match self.data {
            Data::ARP(ref p) => p.to_hex(),
            Data::None => vec![],
        };

        packet.extend(data);

        return packet;
    }
}
