use crate::mem::{Address, Bus};

pub trait Mapper {
    fn cpuRead(memory: &mut dyn Bus, addr: Address) -> Option<u8>;

    fn cpuWrite(memory: &mut dyn Bus, addr: Address, value: u8) -> Option<()>;

    fn ppuRead(memory: &mut dyn Bus, addr: Address) -> Option<u8>;

    fn ppuWrite(memory: &mut dyn Bus, addr: Address, value: u8) -> Option<()>;
}