use std::{
    cmp, fmt,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use byteorder::{ByteOrder, LittleEndian};

use crate::rom::Rom;

#[derive(Debug, Default, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct Address(u16);

impl Address {
    fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#06x}", self.0)
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
        Self(LittleEndian::read_u16(&bytes))
    }
}

impl From<Address> for [u8; 2] {
    fn from(addr: Address) -> Self {
        let mut res = [0; 2];
        LittleEndian::write_u16(&mut res, addr.0);
        res
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

pub struct Memory {
    // 16-bit address space.
    ram: [u8; 65535],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 65535] }
    }

    pub fn load_rom(&mut self, rom: &Rom) {
        let n = cmp::min(self.ram.len(), rom.0.len());
        self.ram[..n].copy_from_slice(&rom.0[..n]);
    }

    pub fn load(&self, addr: Address) -> u8 {
        self.ram[addr.as_usize()]
    }

    pub fn store(&mut self, addr: Address, value: u8) {
        self.ram[addr.as_usize()] = value;
    }
}
