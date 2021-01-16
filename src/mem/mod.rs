use std::cmp;

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

    pub fn load(&self, addr: Address) -> u8 {
        self.ram[addr.as_usize()]
    }

    pub fn store(&mut self, addr: Address, value: u8) {
        self.ram[addr.as_usize()] = value;
    }
}

impl From<&[u8]> for Memory {
    fn from(bytes: &[u8]) -> Self {
        let mut memory = Self::new();
        let n = cmp::min(memory.ram.len(), bytes.len());
        memory.ram[..n].copy_from_slice(&bytes[..n]);
        memory
    }
}
