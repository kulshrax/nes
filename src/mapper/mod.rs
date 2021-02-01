use crate::mem::Address;
use crate::ppu::Vram;
use crate::rom::Rom;

/// Mappers are hardware on the game cartridge that map memory accesses from the
/// CPU and PPU to the cartridge's ROM and RAM chips. By using techniques such
/// as bank switching, mappers can allow the total size of the game's data to be
/// larger than the NES's available address space.
///
/// Since mappers will receive memory accesses from both the CPU and PPU (each
/// of which has its own address space), this trait has separate CPU and PPU
/// methods to make it easy to distinguish the source of the memory access.
pub trait Mapper {
    fn cpu_load(&self, rom: &Rom, addr: Address) -> u8;

    fn cpu_store(&mut self, addr: Address, value: u8);

    fn ppu_load(&self, rom: &Rom, vram: &Vram, addr: Address) -> u8;

    fn ppu_store(&mut self, vram: &mut Vram, addr: Address, value: u8);
}

pub struct Mapper0;

impl Mapper for Mapper0 {
    fn cpu_load(&self, _rom: &Rom, _addr: Address) -> u8 {
        unimplemented!()
    }

    fn cpu_store(&mut self, _addr: Address, _value: u8) {
        unimplemented!()
    }

    fn ppu_load(&self, _rom: &Rom, _vram: &Vram, _addr: Address) -> u8 {
        unimplemented!()
    }

    fn ppu_store(&mut self, _vram: &mut Vram, _addr: Address, _value: u8) {
        unimplemented!()
    }
}
