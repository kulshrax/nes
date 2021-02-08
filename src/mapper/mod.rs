use crate::mem::{Address, Bus};
use crate::ppu::{PpuBus, Vram};
use crate::rom::Rom;

/// CPU mapper trait object that delegates to boxed mapper.
pub type CpuMapper = Box<dyn Bus>;

impl Bus for CpuMapper {
    fn load(&mut self, addr: Address) -> u8 {
        (**self).load(addr)
    }

    fn store(&mut self, addr: Address, value: u8) {
        (**self).store(addr, value)
    }
}

/// PPU mapper trait object that delegates to inner boxed mapper.
pub type PpuMapper = Box<dyn PpuBus>;

impl PpuBus for PpuMapper {
    fn ppu_load(&mut self, vram: &Vram, addr: Address) -> u8 {
        (**self).ppu_load(vram, addr)
    }

    fn ppu_store(&mut self, vram: &mut Vram, addr: Address, value: u8) {
        (**self).ppu_store(vram, addr, value)
    }
}

/// Return the appropriate mapper for this ROM file.
pub fn init_mappers(rom: Rom) -> (CpuMapper, PpuMapper) {
    let (cpu_mapper, ppu_mapper) = Mapper0::from_rom(rom);
    (Box::new(cpu_mapper), Box::new(ppu_mapper))
}

/// Trait representing a cartridge's mapper.
///
/// Mappers are hardware on the game cartridge that map memory accesses from the
/// CPU and PPU to the cartridge's ROM and RAM chips. By using techniques such
/// as bank switching, mappers can allow the total size of the game's data to be
/// larger than the NES's available address space.
///
/// The mapper maps the content of the cartridge into both the CPU and PPU's
/// address space (mapping the contents of the PRG memory into the former and
/// CHR memory into the latter). As such, this trait splits the mapper into
/// a CPU mapper and a PPU mapper, which can share state depending on the
/// implementation, but operate on different address buses.
trait Mapper {
    type CpuMapper: Bus;
    type PpuMapper: PpuBus;

    fn from_rom(rom: Rom) -> (Self::CpuMapper, Self::PpuMapper);
}

struct Mapper0;

impl Mapper for Mapper0 {
    type CpuMapper = CpuMapper0;
    type PpuMapper = PpuMapper0;

    fn from_rom(_rom: Rom) -> (CpuMapper0, PpuMapper0) {
        (CpuMapper0, PpuMapper0)
    }
}

struct CpuMapper0;

impl Bus for CpuMapper0 {
    fn load(&mut self, _addr: Address) -> u8 {
        unimplemented!()
    }

    fn store(&mut self, _addr: Address, _value: u8) {
        unimplemented!()
    }
}

pub struct PpuMapper0;

impl PpuBus for PpuMapper0 {
    fn ppu_load(&mut self, _vram: &Vram, _addr: Address) -> u8 {
        unimplemented!()
    }

    fn ppu_store(&mut self, _vram: &mut Vram, _addr: Address, _value: u8) {
        unimplemented!()
    }
}
