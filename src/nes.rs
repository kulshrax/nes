use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::rom::Rom;

use anyhow::Result;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    rom: Rom,
}

impl Nes {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            rom
        }
    }

    pub fn start(&self) -> Result<()> {
        unimplemented!()
    }
}
