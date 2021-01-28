use crate::mem::Address;
use crate::rom::Rom;

use super::Mapper;

pub struct Mapper0 {
    rom: Rom,
}

impl Mapper0 {
    fn new(rom: Rom) -> Self {
        Self { rom }
    }
}

impl Mapper for Mapper0 {
    fn cpu_load(_addr: Address) -> u8 {
        unimplemented!()
    }

    fn cpu_store(_addr: Address, _value: u8) {
        panic!("Mapper 0 does not support writes!")
    }

    fn ppu_load(_addr: Address) -> u8 {
        unimplemented!()
    }

    fn ppu_store(_addr: Address, _value: u8) {
        panic!("Mapper 0 does not support writes!")
    }
}
