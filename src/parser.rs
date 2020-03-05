use crate::net::{IpAddr, MacAddr};
use std::fmt;
use std::result::Result;

pub struct Parser<'a> {
    input: &'a Vec<u8>,
    index: usize,
}
//#[derive(Debug)]
pub enum ParseError {
    EndOfFile,
    ValidIp(usize),
    ValidMac(usize),
    UnExpect(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EndOfFile => write!(f, "end of file!"),
            ParseError::ValidIp(index) => write!(f, "valid ip at {}", index),
            ParseError::ValidMac(index) => write!(f, "valid mac at {}", index),
            ParseError::UnExpect(s) => write!(f, "unexpect error: {}", s),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a Vec<u8>) -> Self {
        Parser { input, index: 0 }
    }

    pub fn rest(&self) -> Vec<u8> {
        let i = self.index as usize;
        self.input[i..].to_vec()
    }

    pub fn eof(&self) -> bool {
        self.next().is_none()
    }
    pub fn next(&self) -> Option<u8> {
        self.input.get(self.index).cloned()
    }
    pub fn consume_char(&mut self) -> Option<u8> {
        let c = self.next();
        self.index += 1;
        c
    }

    pub fn expect(&mut self, c: u8) -> Result<u8, ParseError> {
        match self.next() {
            Some(ch) if ch == c => {
                self.consume_char();
                return Ok(c);
            }
            Some(ch) => Err(ParseError::UnExpect(format!(
                "expect '{}', but there is a '{}'",
                c, ch
            ))),
            _ => Err(ParseError::EndOfFile),
        }
    }

    pub fn consume_while<F>(&mut self, f: F) -> Vec<u8>
    where
        F: Fn(u8) -> bool,
    {
        let mut result = vec![];
        while self.eof() {
            let c = self.next().unwrap();
            if f(c) {
                result.push(self.consume_char().unwrap());
            } else {
                break;
            }
        }
        result
    }

    pub fn consume_until<F>(&mut self, f: F) -> Vec<u8>
    where
        F: Fn(u8) -> bool,
    {
        let mut result = vec![];
        while self.eof() {
            let c = self.next().unwrap();
            if !f(c) {
                result.push(self.consume_char().unwrap());
            } else {
                break;
            }
        }
        result
    }

    pub fn parse_u8(&mut self) -> Result<u8, ParseError> {
        match self.next() {
            Some(s1) => {
                self.index += 1;
                Ok(s1)
            }
            None => Err(ParseError::EndOfFile),
        }
    }

    pub fn parse_u16(&mut self) -> Result<u16, ParseError> {
        let h = self.parse_u8()?;
        let l = self.parse_u8()?;
        Ok((h * 16 + l) as u16)
    }

    pub fn parse_mac(&mut self) -> Result<MacAddr, ParseError> {
        let a = self.parse_u8()?;
        let b = self.parse_u8()?;
        let c = self.parse_u8()?;
        let d = self.parse_u8()?;
        let e = self.parse_u8()?;
        let f = self.parse_u8()?;
        return Ok(MacAddr(a, b, c, d, e, f));
    }

    pub fn parse_ip_v4(&mut self) -> Result<IpAddr, ParseError> {
        let a = self.parse_u8()?;
        let b = self.parse_u8()?;
        let c = self.parse_u8()?;
        let d = self.parse_u8()?;
        return Ok(IpAddr(a, b, c, d));
    }
}
