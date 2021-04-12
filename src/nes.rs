use std::time::Duration;

use anyhow::Result;
use winit_input_helper::WinitInputHelper;

use crate::cpu::Cpu;
use crate::mapper::{self, CpuMapper, PpuMapper};
use crate::mem::{Address, Memory, Ram};
use crate::ppu::{Ppu, FRAME_HEIGHT, FRAME_WIDTH};
use crate::rom::Rom;
use crate::ui::Ui;

const CPU_CYCLES_PER_FRAME: usize = 29781;

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

    /// Run the CPU only without any visual output.
    pub fn run_cpu(&mut self, start: Option<Address>) {
        if let Some(start) = start {
            self.cpu.set_pc(start);
        }
        loop {
            let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.mapper);
            self.cpu.step(&mut memory);
        }
    }

    /// Run the system for the duration of a single frame, writing the contents
    /// of the new frame to the give frame buffer.
    pub fn run_one_frame(&mut self, frame: &mut [u8], _input: &WinitInputHelper) {
        for i in 0..CPU_CYCLES_PER_FRAME {
            if i % 1000 == 0 {
                log::debug!("cycle {}", i);
            }
            // Create a view of the CPU's addres space, including all memory-mapped devices.
            let mut memory = Memory::new(&mut self.ram, &mut self.ppu, &mut self.mapper);

            // Run the CPU.
            self.cpu.tick(&mut memory);

            // // Run the PPU. The PPU's clock runs 3x faster than the CPU's.
            // for _ in 0..3 {
            // }
        }
        self.ppu.tick(frame);
    }
}

impl Ui for Nes {
    fn size(&self) -> (u32, u32) {
        (FRAME_WIDTH as u32, FRAME_HEIGHT as u32)
    }

    fn update(&mut self, frame: &mut [u8], input: &WinitInputHelper, _dt: Duration) -> Result<()> {
        self.run_one_frame(frame, input);
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::VecDeque;
    use std::env;
    use std::path::PathBuf;

    use crate::rom::Rom;

    #[test]
    fn nestest() {
        let manifest_dir: PathBuf = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR environment variable not set")
            .into();

        // Load the "nestest" ROM, which is a comprehensive CPU test.
        let nestest = manifest_dir.join("data/nestest/nestest.nes");
        let rom = Rom::load(nestest).expect("Failed to load nestest ROM");
        let mut nes = Nes::new(rom);

        // Manually set the starting address to 0xC000, which is the intended
        // entry point for running the ROM in a headless/automated context.
        nes.cpu.set_pc(Address(0xC000));

        // Load a log file containing the expected program counter values for
        // each step of executing the test. This log is derived from a run of
        // this ROM on the Nintendulator emulator, whose CPU is known to work
        // correctly. The log was obtained from the [NesDev wiki].
        //
        // [NesDev wiki]: https://wiki.nesdev.com/w/index.php/Emulator_tests
        let mut expected_pcs = VecDeque::new();
        let log = include_str!("../data/nestest/nestest.log");
        for line in log.lines() {
            // The first 4 charcters of each line are program counter values.
            expected_pcs.push_back(line[..4].parse().unwrap());
        }

        // Run the CPU until we reach the end of the log.
        while let Some(expected) = expected_pcs.pop_front() {
            assert_eq!(nes.cpu.registers().pc, expected);
            let mut memory = Memory::new(&mut nes.ram, &mut nes.ppu, &mut nes.mapper);
            // Don't check cycle timings.
            let _ = nes.cpu.step(&mut memory);
        }
    }
}
