use crate::mem::{Address, Bus};
use crate::ppu::{PpuBus, Vram};
use crate::rom::Rom;

mod mapper0;

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

/// Initialize the appropriate mappers for this ROM file.
pub fn init(rom: Rom) -> (CpuMapper, PpuMapper) {
    // TODO: Read mapper number from ROM header to select appropriate mapper.
    let (cpu_mapper, ppu_mapper) = mapper0::Mapper0::from_rom(rom);
    (Box::new(cpu_mapper), Box::new(ppu_mapper))
}

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
