use std::borrow::Cow;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::str::FromStr;

use anyhow::{anyhow, bail, Context, Error};
use hex::FromHex;

#[derive(Default, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address(pub u16);

impl Address {
    /// Convert address into a usize so that it can be used as an array index.
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }

    /// Mask off the most significant bits of the address, leaving the specified
    /// number of trailing bits intact. This simulates incomplete decoding of
    /// addresses (wherein only the N least significant address lines are read
    /// by the NES hardware), resulting in aliasing that causes regions of
    /// memory to be mirrored throughout the address space.
    pub fn alias(&self, n_bits: u8) -> Address {
        let mask = (1 << n_bits) - 1;
        Address(self.0 & mask)
    }

    /// Get the raw little-endian bytes of this address.
    pub fn to_le_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06X}", self.0)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address({:#06X})", self.0)
    }
}

impl From<u16> for Address {
    fn from(addr: u16) -> Self {
        Self(addr)
    }
}

/// An address can be constructed from a single byte, in which case that byte
/// will be used as the least significant byte of the 16-bit address. This is
/// useful for addresses in the zero page (the first 256 bytes of memory).
impl From<u8> for Address {
    fn from(addr: u8) -> Self {
        Self(addr as u16)
    }
}

/// Interpret bytes as a little-endian 16-bit address.
impl From<[u8; 2]> for Address {
    fn from(bytes: [u8; 2]) -> Self {
        Self(u16::from_le_bytes(bytes))
    }
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Ensure the input string is 4 bytes long.
        let hex = s.strip_prefix("0x").unwrap_or(s);
        let hex = match hex.len() {
            0 => bail!("Empty address"),
            1 => Cow::from(format!("000{}", hex)),
            2 => Cow::from(format!("00{}", hex)),
            3 => Cow::from(format!("0{}", hex)),
            4 => Cow::from(hex),
            _ => bail!("Address is longer than 16 bits: {:?}", s),
        };

        let addr = <[u8; 2]>::from_hex(hex.as_ref())
            .map(u16::from_be_bytes)
            .with_context(|| anyhow!("Invalid hex address: {:?}", s))?;

        Ok(Address(addr))
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
        Self(self.0.wrapping_add(other as u16))
    }
}

impl Sub<u8> for Address {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        Self(self.0.wrapping_sub(other as u16))
    }
}

impl Add<i8> for Address {
    type Output = Self;

    fn add(self, other: i8) -> Self {
        let other = other as i16;
        if other < 0 {
            Self(self.0.wrapping_sub(-other as u16))
        } else {
            Self(self.0.wrapping_add(other as u16))
        }
    }
}

impl Sub<i8> for Address {
    type Output = Self;

    fn sub(self, other: i8) -> Self {
        if other < 0 {
            Self(self.0.wrapping_add(-other as u16))
        } else {
            Self(self.0.wrapping_sub(other as u16))
        }
    }
}

impl AddAssign<u8> for Address {
    fn add_assign(&mut self, other: u8) {
        self.0 = self.0.wrapping_add(other as u16);
    }
}

impl SubAssign<u8> for Address {
    fn sub_assign(&mut self, other: u8) {
        self.0 = self.0.wrapping_sub(other as u16);
    }
}

impl AddAssign<i8> for Address {
    fn add_assign(&mut self, other: i8) {
        if other < 0 {
            self.0 = self.0.wrapping_sub(-other as u16);
        } else {
            self.0 = self.0.wrapping_add(other as u16);
        }
    }
}

impl SubAssign<i8> for Address {
    fn sub_assign(&mut self, other: i8) {
        if other < 0 {
            self.0 = self.0.wrapping_add(-other as u16);
        } else {
            self.0 = self.0.wrapping_sub(other as u16);
        }
    }
}

impl Add<u16> for Address {
    type Output = Self;

    fn add(self, other: u16) -> Self {
        Self(self.0.wrapping_add(other))
    }
}

impl Sub<u16> for Address {
    type Output = Self;

    fn sub(self, other: u16) -> Self {
        Self(self.0.wrapping_sub(other))
    }
}

impl Add<i16> for Address {
    type Output = Self;

    fn add(self, other: i16) -> Self {
        let other = other as i16;
        if other < 0 {
            Self(self.0.wrapping_sub(-other as u16))
        } else {
            Self(self.0.wrapping_add(other as u16))
        }
    }
}

impl Sub<i16> for Address {
    type Output = Self;

    fn sub(self, other: i16) -> Self {
        if other < 0 {
            Self(self.0.wrapping_add(-other as u16))
        } else {
            Self(self.0.wrapping_sub(other as u16))
        }
    }
}

impl AddAssign<u16> for Address {
    fn add_assign(&mut self, other: u16) {
        self.0 = self.0.wrapping_add(other);
    }
}

impl SubAssign<u16> for Address {
    fn sub_assign(&mut self, other: u16) {
        self.0 = self.0.wrapping_sub(other);
    }
}

impl AddAssign<i16> for Address {
    fn add_assign(&mut self, other: i16) {
        if other < 0 {
            self.0 = self.0.wrapping_sub(-other as u16);
        } else {
            self.0 = self.0.wrapping_add(other as u16);
        }
    }
}

impl SubAssign<i16> for Address {
    fn sub_assign(&mut self, other: i16) {
        if other < 0 {
            self.0 = self.0.wrapping_add(-other as u16);
        } else {
            self.0 = self.0.wrapping_sub(other as u16);
        }
    }
}

impl Add<usize> for Address {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        debug_assert!(other <= u16::MAX as usize);
        Self(self.0.wrapping_add(other as u16))
    }
}

impl Sub<usize> for Address {
    type Output = Self;

    fn sub(self, other: usize) -> Self {
        debug_assert!(other <= u16::MAX as usize);
        Self(self.0.wrapping_sub(other as u16))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    #[test]
    fn test_address_parsing() -> Result<()> {
        let addr: Address = "0x0400".parse()?;
        assert_eq!(addr, Address(0x400));

        let truncated: Address = "0x400".parse()?;
        assert_eq!(truncated, Address(0x400));

        let one_byte: Address = "0x42".parse()?;
        assert_eq!(one_byte, Address(0x42));

        let truncated_one_byte: Address = "0x042".parse()?;
        assert_eq!(truncated_one_byte, Address(0x042));

        let no_hex_prefix: Address = "400".parse()?;
        assert_eq!(no_hex_prefix, Address(0x400));

        let too_big: Result<Address> = "0xDEADBEEF".parse();
        assert!(too_big.is_err());

        let not_a_number: Result<Address> = "invalid".parse();
        assert!(not_a_number.is_err());

        Ok(())
    }
}
