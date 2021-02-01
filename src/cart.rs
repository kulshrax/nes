use crate::mapper::{Mapper, Mapper0};
use crate::mem::{Address, Bus};
use crate::ppu::{PpuBus, Vram};
use crate::rom::Rom;

pub struct Cartridge {
    rom: Rom,
    mapper: Box<dyn Mapper>,
}

impl Cartridge {
    pub fn new(rom: Rom) -> Self {
        Self {
            rom,
            mapper: Box::new(Mapper0),
        }
    }
}

impl Bus for Cartridge {
    fn load(&self, addr: Address) -> u8 {
        self.mapper.cpu_load(&self.rom, addr)
    }

    fn store(&mut self, addr: Address, value: u8) {
        self.mapper.cpu_store(addr, value)
    }
}

impl PpuBus for Cartridge {
    fn load(&self, vram: &Vram, addr: Address) -> u8 {
        self.mapper.ppu_load(&self.rom, vram, addr)
    }

    fn store(&mut self, vram: &mut Vram, addr: Address, value: u8) {
        self.mapper.ppu_store(vram, addr, value)
    }
}
