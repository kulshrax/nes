use std::path::Path;

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

    pub fn run(&mut self, path: impl AsRef<Path>, start: Address) {
        self.memory.load_raw_binary(path);
        self.cpu.set_init(&mut self.memory, start);
        self.cpu.reset(&mut self.memory);
        let mut old_pc = start;
        loop {
            let pc = self.cpu.step(&mut self.memory);
            if pc == old_pc {
                log::error!("Detected infinite loop at {}; stopping CPU", pc);
                log::error!("Registers: {}", self.cpu.registers());
                break;
            }
            old_pc = pc;
        }
    }
}
