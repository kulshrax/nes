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
}
