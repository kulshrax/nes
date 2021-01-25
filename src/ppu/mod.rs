use crate::mem::{Address, Bus};

pub struct Ppu {
    ram: [u8; 2048],
    oam: [u8; 256],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ram: [0u8; 2048],
            oam: [0u8; 256],
        }
    }
}

impl Bus for Ppu {
    fn load(&self, addr: Address) -> u8 {
        unimplemented!()
    }

    fn store(&mut self, addr: Address, value: u8) {
        unimplemented!()
    }
}
