use crate::cpu::Cpu;

pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
        }
    }
}
