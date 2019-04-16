use std::cmp;

use crate::rom::Rom;

pub use address::Address;

mod address;

pub struct Memory {
    // 16-bit address space.
    ram: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 0x10000] }
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
