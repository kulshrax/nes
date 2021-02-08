use crate::mem::{Address, Bus};
use crate::ppu::{PpuBus, Vram};
use crate::rom::Rom;

use super::Mapper;

pub(super) struct Mapper0;

impl Mapper for Mapper0 {
    type CpuMapper = CpuMapper0;
    type PpuMapper = PpuMapper0;

    fn from_rom(_rom: Rom) -> (CpuMapper0, PpuMapper0) {
        (CpuMapper0, PpuMapper0)
    }
}

pub(super) struct CpuMapper0;

impl Bus for CpuMapper0 {
    fn load(&mut self, _addr: Address) -> u8 {
        unimplemented!()
    }

    fn store(&mut self, _addr: Address, _value: u8) {
        unimplemented!()
    }
}

pub(super) struct PpuMapper0;

impl PpuBus for PpuMapper0 {
    fn ppu_load(&mut self, _vram: &Vram, _addr: Address) -> u8 {
        unimplemented!()
    }

    fn ppu_store(&mut self, _vram: &mut Vram, _addr: Address, _value: u8) {
        unimplemented!()
    }
}
