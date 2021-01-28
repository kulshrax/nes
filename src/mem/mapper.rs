use crate::mem::{Address, Bus};

pub trait Mapper {
    fn cpuLoad(memory: &mut dyn Bus, addr: Address) -> Option<u8>;

    fn cpuStore(memory: &mut dyn Bus, addr: Address, value: u8) -> Option<()>;

    fn ppuLoad(memory: &mut dyn Bus, addr: Address) -> Option<u8>;

    fn ppuStore(memory: &mut dyn Bus, addr: Address, value: u8) -> Option<()>;
}
