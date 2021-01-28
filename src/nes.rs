use crate::cpu::Cpu;
use crate::mem::Memory;
use crate::rom::Rom;

use anyhow::Result;

pub struct Nes {
    cpu: Cpu,
    mem: Memory,
}

impl Nes {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu: Cpu::new(),
            mem: Memory::new(rom),
        }
    }

    pub fn start(&self) -> Result<()> {
        unimplemented!()
    }
}
