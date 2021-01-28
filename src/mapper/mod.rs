use crate::mem::Address;

pub trait Mapper {
    fn cpu_load(addr: Address) -> u8;

    fn cpu_store(addr: Address, value: u8);

    fn ppu_load(addr: Address) -> u8;

    fn ppu_store(addr: Address, value: u8);
}
