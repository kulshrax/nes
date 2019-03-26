use crate::cpu::Cpu;
use crate::mem::Memory;
use crate::rom::Rom;

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

    pub fn run(&mut self, rom: &Rom) {
        self.memory.load(rom);
        self.cpu.set_pc(0);
        loop {
            self.cpu.step(&mut self.memory);
        }
    }
}
