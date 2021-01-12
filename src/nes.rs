use std::path::Path;

use anyhow::Result;

use crate::cpu::Cpu;
use crate::mem::{Address, Memory};

pub struct Nes {
    cpu: Cpu,
    memory: Memory,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self, path: impl AsRef<Path>, start: Address) -> Result<()> {
        self.memory.load_raw_binary(path);
        self.cpu.set_reset_vector(&mut self.memory, start);
        self.cpu.reset(&mut self.memory);

        loop {
            self.cpu.step(&mut self.memory)?;
        }
    }
}
