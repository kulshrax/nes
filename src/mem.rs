use std::{cmp, fs::File, io::prelude::*, path::Path};

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

    pub fn load_file(&mut self, path: impl AsRef<Path>) {
        let mut buf = Vec::new();
        let mut f = File::open(path.as_ref()).unwrap();
        let file_size = f.read_to_end(&mut buf).unwrap();

        let n = cmp::min(self.ram.len(), file_size);
        self.ram[..n].copy_from_slice(&buf[..n]);
    }

    pub fn load(&self, addr: Address) -> u8 {
        self.ram[addr.as_usize()]
    }

    pub fn store(&mut self, addr: Address, value: u8) {
        self.ram[addr.as_usize()] = value;
    }
}
