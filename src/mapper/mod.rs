use crate::mem::Address;

mod mapper0;

/// Mappers are hardware on the game cartridge that map memory accesses from the
/// CPU and PPU to the cartridge's ROM and RAM chips. By using techniques such
/// as bank switching, mappers can allow the total size of the game's data to be
/// larger than the NES's available address space.
///
/// Since mappers will receive memory accesses from both the CPU and PPU (each
/// of which has its own address space), this trait has separate CPU and PPU
/// methods to make it easy to distinguish the source of the memory access.
pub trait Mapper {
    fn cpu_load(addr: Address) -> u8;

    fn cpu_store(addr: Address, value: u8);

    fn ppu_load(addr: Address) -> u8;

    fn ppu_store(addr: Address, value: u8);
}
