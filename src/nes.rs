use anyhow::Result;

use crate::cpu::Cpu;
use crate::mapper::{self, CpuMapper, PpuMapper};
use crate::mem::{Memory, Ram};
use crate::ppu::Ppu;
use crate::rom::Rom;
use crate::ui::Ui;

pub struct Nes {
    cpu: Cpu,
    ram: Ram,
    ppu: Ppu<PpuMapper>,
    mapper: CpuMapper,
}

impl Nes {
    pub fn new(rom: Rom) -> Self {
        let (mapper, ppu_mapper) = mapper::init(rom);
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(),
            ppu: Ppu::with_mapper(ppu_mapper),
            mapper,
        }
    }

    pub fn poll(&mut self, ui: Ui) -> Result<()> {
        //        // Create a view of the CPU's addres space, including all memory-mapped devices.
        //        let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.mapper);
        //
        //        // Run the CPU.
        //        self.cpu.tick(&mut memory)?;
        //
        //        // Run the PPU. The PPU's clock runs 3x faster than the CPU's.
        //        for _ in 0..3 {
        //            self.ppu.tick(ui.frame);
        //        }

        self.ppu.read_pattern_table(ui.frame.get_frame());

        Ok(())
    }
}
