use std::time::Duration;

use anyhow::Result;
use winit_input_helper::WinitInputHelper;

use crate::cpu::Cpu;
use crate::mapper::{self, CpuMapper, PpuMapper};
use crate::mem::{Address, Memory, Ram};
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
        let (mut mapper, ppu_mapper) = mapper::init(rom);

        let mut cpu = Cpu::new();
        let mut ram = Ram::new();
        let mut ppu = Ppu::with_mapper(ppu_mapper);

        // Reset the CPU to set the initial value of the program counter from
        // the reset vector (loaded from memory via the CPU mapper).
        let mut memory = Memory::new(&mut ram, &mut ppu, &mut mapper);
        cpu.reset(&mut memory);

        Self {
            cpu,
            ram,
            ppu,
            mapper,
        }
    }

    pub fn step(&mut self) {
        // Create a view of the CPU's addres space, including all memory-mapped devices.
        let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.mapper);

        // Run the CPU.
        self.cpu.tick(&mut memory);
    }

    pub fn run_headless(&mut self) {
        self.cpu.set_pc(Address(0xC000));
        loop {
            self.step();
        }
    }
}

impl Ui for Nes {
    fn size(&self) -> (u32, u32) {
        (FRAME_WIDTH as u32, FRAME_HEIGHT as u32)
    }

    fn update(&mut self, frame: &mut [u8], _input: &WinitInputHelper, _dt: Duration) -> Result<()> {
        for _ in 0..1000 {
            // Create a view of the CPU's addres space, including all memory-mapped devices.
            let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.mapper);

            // Run the CPU.
            self.cpu.tick(&mut memory);

            // Run the PPU. The PPU's clock runs 3x faster than the CPU's.
            for _ in 0..3 {
                self.ppu.tick(frame);
            }
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
