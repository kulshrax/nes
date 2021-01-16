use std::{
    borrow::Cow,
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

use anyhow::{bail, Context, Error};
use hex::FromHex;

#[derive(Default, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address(u16);

impl Address {
    /// Convert this address into a usize so that it
    /// can be used as an array index.
    pub(super) fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06x}", self.0)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({:#06x})", self.0)
    }
}

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Self(addr)
    }
}

/// An address can be constructed from a single byte, in
/// which case that byte will be used as the least significant
/// byte of the 16-bit address. This is useful for constructing
/// addresses for the zero page (the first 256 bytes of memory).
impl From<u8> for Address {
    fn from(addr: u8) -> Self {
        Self(addr as u16)
    }
}

/// When constructing an address from an array of two bytes,
/// the bytes will be interpretted in little endian order.
impl From<[u8; 2]> for Address {
    fn from(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(addr: &str) -> Result<Self, Self::Err> {
        Ok(Address(match addr.strip_prefix("0x") {
            // Parse has hex.
            Some(hex) => {
                // Ensure the input string is 4 bytes long.
                let hex = match hex.len() {
                    0 => bail!("Empty address"),
                    1 => Cow::from(format!("000{}", hex)),
                    2 => Cow::from(format!("00{}", hex)),
                    3 => Cow::from(format!("0{}", hex)),
                    4 => Cow::from(hex),
                    _ => bail!("Address is longer than 16 bits: {:?}", addr),
                };
                <[u8; 2]>::from_hex(hex.as_ref())
                    .map(u16::from_be_bytes)
                    .with_context(|| format!("Invalid hex address: {:?}", addr))?
            }
            // Parse as decimal.
            None => addr
                .parse()
                .with_context(|| format!("Invalid decimal address: {:?}", addr))?,
        }))
    }
}

impl From<Address> for [u8; 2] {
    fn from(addr: Address) -> Self {
        addr.0.to_le_bytes()
    }
}

impl Add<u8> for Address {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        Self(self.0 + other as u16)
    }
}

impl Sub<u8> for Address {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        Self(self.0 - other as u16)
    }
}

impl Add<i8> for Address {
    type Output = Self;

    fn add(self, other: i8) -> Self {
        let other = other as i16;
        if other < 0 {
            Self(self.0 - -other as u16)
        } else {
            Self(self.0 + other as u16)
        }
    }
}

impl Sub<i8> for Address {
    type Output = Self;

    fn sub(self, other: i8) -> Self {
        if other < 0 {
            Self(self.0 + -other as u16)
        } else {
            Self(self.0 - other as u16)
        }
    }
}

impl AddAssign<u8> for Address {
    fn add_assign(&mut self, other: u8) {
        self.0 += other as u16;
    }
}

impl SubAssign<u8> for Address {
    fn sub_assign(&mut self, other: u8) {
        self.0 -= other as u16;
    }
}

impl AddAssign<i8> for Address {
    fn add_assign(&mut self, other: i8) {
        if other < 0 {
            self.0 -= -other as u16;
        } else {
            self.0 += other as u16;
        }
    }
}

impl SubAssign<i8> for Address {
    fn sub_assign(&mut self, other: i8) {
        if other < 0 {
            self.0 += -other as u16;
        } else {
            self.0 -= other as u16;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    #[test]
    fn test_address_parsing() -> Result<()> {
        let hex: Address = "0x0400".parse()?;
        assert_eq!(hex, Address(0x400));

        let odd_hex: Address = "0x400".parse()?;
        assert_eq!(odd_hex, Address(0x400));

        let short_hex: Address = "0x42".parse()?;
        assert_eq!(short_hex, Address(0x42));

        let odd_short_hex: Address = "0x042".parse()?;
        assert_eq!(odd_short_hex, Address(0x042));

        let decimal: Address = "400".parse()?;
        assert_eq!(decimal, Address(400));

        let too_big_hex: Result<Address> = "0xDEADBEEF".parse();
        assert!(too_big_hex.is_err());

        let too_big_decimal: Result<Address> = "123456789".parse();
        assert!(too_big_decimal.is_err());

        let not_a_number: Result<Address> = "invalid".parse();
        assert!(not_a_number.is_err());

        Ok(())
    }
}
