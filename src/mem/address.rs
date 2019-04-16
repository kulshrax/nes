use std::{
    fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

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
