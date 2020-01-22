use std::fmt::{Display, Formatter, Error};
use std::ops::BitAnd;

#[derive(Copy, Clone)]
pub struct IpAddress {
    pub bytes: [u8; 4]
}

impl IpAddress {
    pub fn new(one: u8, two: u8, three: u8, four: u8) -> IpAddress {
        IpAddress {
            bytes: [one, two, three, four]
        }
    }

    pub fn parse_ip(inp: &str) -> Option<IpAddress> {
        let parts: Vec<Option<u8>> = inp.split(".").map(|x| x.parse::<u8>().ok()).collect();
        if parts.contains(&None) {
            return None;
        }
        if parts.len() != 4 {
            return None;
        }
        let one = parts.get(0).unwrap().unwrap();
        let two = parts.get(1).unwrap().unwrap();
        let three = parts.get(2).unwrap().unwrap();
        let four = parts.get(3).unwrap().unwrap();

        Some(IpAddress::new(one, two, three, four))
    }

    pub fn subnetmask(length: u8) -> Option<IpAddress> {
        if length > 32 {
            return None;
        }
        let mut bytes: [u8; 4] = [0; 4];
        let full_bytes = (length / 8) as usize;
        let remainder = (length % 8) as usize;
        for i in 0..full_bytes {
            bytes[i] = 255
        }
        for i in 0..remainder {
            bytes[full_bytes] += (1 << i) as u8
        }
        bytes[full_bytes] <<= ((8 - remainder) % 8) as u8;
        Some(IpAddress { bytes })
    }
}

impl BitAnd for IpAddress {
    type Output = IpAddress;

    fn bitand(self, rhs: Self) -> Self::Output {
        IpAddress {
            bytes: [
                self.bytes[0] & rhs.bytes[0],
                self.bytes[1] & rhs.bytes[1],
                self.bytes[2] & rhs.bytes[2],
                self.bytes[3] & rhs.bytes[3]
            ]
        }
    }
}

impl Display for IpAddress {
    fn fmt(&self, w: &mut Formatter<'_>) -> Result<(), Error> {
        write!(w, "{}.{}.{}.{}", self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3])
    }
}