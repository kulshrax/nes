use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::rom::Rom;
use crate::mem::Memory;

use anyhow::Result;

pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
    rom: Rom,
    mem: Memory,
}

impl Nes {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu: Cpu::new(),
            ppu: Ppu::new(),
            mem: Memory::new(),
            rom,
        }
    }

    pub fn start(&self) -> Result<()> {
        unimplemented!()
    }
}
