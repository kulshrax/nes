use std::cmp;

use crate::rom::Rom;

pub type Address = u16;

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
        self.ram[addr as usize]
    }

    /// Interpret the two bytes starting at the given address
    /// as a 16-bit little-endian memory address.
    ///
    /// Used exclusively by the JMP instruction's indirect
    /// addresssing mode, wherein the address to jump to is
    /// read from memory.
    pub fn load_addr(&mut self, addr: Address) -> Address {
        let lsb = self.ram[addr as usize] as u16;
        let msb = self.ram[addr as usize + 1] as u16;
        (msb << 8) | lsb
    }

    pub fn store(&mut self, addr: Address, value: u8) {
        self.ram[addr as usize] = value;
    }
}
