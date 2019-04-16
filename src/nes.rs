use crate::cpu::Cpu;
use crate::mem::{Address, Memory};
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

    pub fn run(&mut self, rom: &Rom, start: Address) {
        self.memory.load_rom(rom);
        self.cpu.set_init(&mut self.memory, start);
        self.cpu.reset(&mut self.memory);
        let mut old_pc = start;
        loop {
            let pc = self.cpu.step(&mut self.memory);
            if pc == old_pc {
                log::error!("Detected infinite loop at {}; stopping CPU", pc);
                log::error!("Registers: {}", self.cpu.dump_registers());
                break;
            }
            old_pc = pc;
        }
    }
}
