use crate::net::{IpAddr, MacAddr, Serializable};
use crate::parser;
use crate::parser::{ParseError, Parser};
use std::fmt;
use std::fmt::{Display, Error, Formatter};

enum HardwareType {
    Ether,
}

impl Display for HardwareType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ether")
    }
}

impl Serializable for HardwareType {
    fn to_hex(&self) -> Vec<u8> {
        match *self {
            HardwareType::Ether => vec![0x00, 0x01],
        }
    }
}

enum ProtocolType {
    IPV4,
}

impl fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ipv4")
    }
}

impl Serializable for ProtocolType {
    fn to_hex(&self) -> Vec<u8> {
        match *self {
            ProtocolType::IPV4 => vec![0x08, 0x00],
        }
    }
}

pub enum Operation {
    Request,
    Reply,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Operation::Request => write!(f, "Request"),
            Operation::Reply => write!(f, "Reply"),
        }
    }
}

impl Serializable for Operation {
    fn to_hex(&self) -> Vec<u8> {
        match *self {
            Operation::Request => vec![0x00, 0x01],
            Operation::Reply => vec![0x00, 0x02],
        }
    }
}

pub struct Packet {
    hardware_type: HardwareType,
    protocol_type: ProtocolType,
    hardware_size: u8,
    protocol_size: u8,
    op: Operation,
    sender_mac: MacAddr,
    sender_ip: IpAddr,
    target_mac: MacAddr,
    target_ip: IpAddr,
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "hardware type:   {}\n", self.hardware_type);
        write!(f, "protocol type:   {}\n", self.protocol_type);
        write!(f, "hardware size:   {}\n", self.hardware_size);
        write!(f, "protocol_size:   {}\n", self.protocol_size);
        write!(f, "operation:       {}\n", self.op);
        write!(f, "sender mac:      {}\n", self.sender_mac);
        write!(f, "sender ip:       {}\n", self.sender_ip);
        write!(f, "target mac:      {}\n", self.target_mac);
        write!(f, "target ip:       {}\n", self.target_ip)
    }
}

impl<'a> Parser<'a> {
    fn parse_hardware_type(&mut self) -> Result<HardwareType, ParseError> {
        // TODO: add other hardware type, only support ether now
        let hex_pair = HardwareType::Ether.to_hex();
        let _ = self.expect(hex_pair[0])?;
        let _ = self.expect(hex_pair[1])?;
        Ok(HardwareType::Ether)
    }

    fn parse_protocol_type(&mut self) -> Result<ProtocolType, ParseError> {
        let hex_pair = ProtocolType::IPV4.to_hex();
        let _ = self.expect(hex_pair[0])?;
        let _ = self.expect(hex_pair[1])?;
        Ok(ProtocolType::IPV4)
    }

    fn parse_operation(&mut self) -> Result<Operation, ParseError> {
        let op_code = self.parse_u16()?;
        match op_code {
            code if code == 1 => Ok(Operation::Request),
            code if code == 2 => Ok(Operation::Reply),
            _ => Err(ParseError::UnExpect(
                format!("unrecognized operation code! : {}", op_code).to_string(),
            )),
        }
    }
}

impl Packet {
    pub fn parse(packet: &Vec<u8>) -> Result<Packet, ParseError> {
        let mut parser = parser::Parser::new(packet);
        let hardware_type = parser.parse_hardware_type()?;
        let protocol_type = parser.parse_protocol_type()?;
        let hardware_size = parser.parse_u8()?;
        let protocol_size = parser.parse_u8()?;
        let op = parser.parse_operation()?;
        let sender_mac = parser.parse_mac()?;
        let sender_ip = parser.parse_ip_v4()?;
        let target_mac = parser.parse_mac()?;
        let target_ip = parser.parse_ip_v4()?;
        Ok(Packet {
            hardware_type,
            protocol_type,
            hardware_size,
            protocol_size,
            op,
            sender_mac,
            sender_ip,
            target_mac,
            target_ip,
        })
    }

    pub fn new() -> Self {
        let packet = Packet {
            hardware_type: HardwareType::Ether,
            protocol_type: ProtocolType::IPV4,
            hardware_size: 6,
            protocol_size: 4,
            op: Operation::Request,
            sender_mac: MacAddr(0, 0, 0, 0, 0, 0),
            sender_ip: IpAddr(0, 0, 0, 0),
            target_mac: MacAddr(0, 0, 0, 0, 0, 0),
            target_ip: IpAddr(0, 0, 0, 0),
        };
        packet
    }

    pub fn set_operation(&mut self, op: Operation) {
        self.op = op;
    }

    pub fn set_sender_mac(&mut self, mac: MacAddr) {
        self.sender_mac = mac;
    }

    pub fn set_sender_ip(&mut self, ip: IpAddr) {
        self.sender_ip = ip;
    }

    pub fn set_target_mac(&mut self, mac: MacAddr) {
        self.target_mac = mac;
    }

    pub fn set_target_ip(&mut self, ip: IpAddr) {
        self.target_ip = ip;
    }
}

impl Serializable for Packet {
    fn to_hex(&self) -> Vec<u8> {
        let mut packet: Vec<u8> = vec![];
        packet.extend(self.hardware_type.to_hex());
        packet.extend(self.protocol_type.to_hex());
        packet.push(self.hardware_size);
        packet.push(self.protocol_size);
        packet.extend(self.op.to_hex());
        packet.extend(self.sender_mac.to_hex());
        packet.extend(self.sender_ip.to_hex());
        packet.extend(self.target_mac.to_hex());
        packet.extend(self.target_ip.to_hex());
        packet
    }
}
