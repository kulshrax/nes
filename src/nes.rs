use crate::cart::Cartridge;
use crate::cpu::Cpu;
use crate::mem::{Memory, Ram};
use crate::ppu::Ppu;
use crate::rom::Rom;

use anyhow::Result;

pub struct Nes {
    cpu: Cpu,
    ram: Ram,
    ppu: Ppu,
    cart: Cartridge,
}

impl Nes {
    pub fn new(rom: Rom) -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            ppu: Ppu::new(),
            cart: Cartridge::new(rom),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.cart);
            self.cpu.tick(&mut memory)?;

            // The PPU's clock runs 3x faster than the CPU's.
            for _ in 0..3 {
                self.ppu.tick(&mut self.cart);
            }
        }
    }
}
