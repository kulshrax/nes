use std::time::Duration;

use anyhow::Result;
use winit_input_helper::WinitInputHelper;

use crate::cpu::Cpu;
use crate::mapper::{self, CpuMapper, PpuMapper};
use crate::mem::{Memory, Ram};
use crate::ppu::{Ppu, FRAME_HEIGHT, FRAME_WIDTH};
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
}

impl Ui for Nes {
    fn size(&self) -> (u32, u32) {
        (FRAME_WIDTH as u32, FRAME_HEIGHT as u32)
    }

    fn update(&mut self, frame: &mut [u8], _input: &WinitInputHelper, _dt: Duration) -> Result<()> {
        // Create a view of the CPU's addres space, including all memory-mapped devices.
        let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.mapper);

        // Run the CPU.
        self.cpu.tick(&mut memory)?;

        // Run the PPU. The PPU's clock runs 3x faster than the CPU's.
        for _ in 0..3 {
            self.ppu.tick(frame);
        }

        Ok(())
    }
}

/// Newtype wrapper to provide alternative UI for show-pattern command.
pub struct ShowPatternUi {
    nes: Nes,
}

impl ShowPatternUi {
    pub fn new(nes: Nes) -> Self {
        ShowPatternUi { nes }
    }
}

impl Ui for ShowPatternUi {
    fn size(&self) -> (u32, u32) {
        // Enough space to render both pattern tables (128x128) side-by-side.
        (256, 128)
    }

    fn update(&mut self, frame: &mut [u8], _input: &WinitInputHelper, _dt: Duration) -> Result<()> {
        self.nes.ppu.render_pattern_table(frame);
        Ok(())
    }
}
