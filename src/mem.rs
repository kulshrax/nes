use std::cmp;

use crate::rom::Rom;

pub type Address = u16;

pub struct Memory {
    ram: [u8; 2048],
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; 2048] }
    }

    pub fn load(&mut self, rom: &Rom) {
        let n = cmp::min(self.ram.len(), rom.0.len());
        self.ram[..n].copy_from_slice(&rom.0[..n]);
    }
}
