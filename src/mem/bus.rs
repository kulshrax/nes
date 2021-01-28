use super::address::Address;

/// Trait representing the CPU's address bus. The actual destination of loads
/// and stores are mapped by hardware to several possible locations, including
/// the NES's RAM, the PPU, various IO registers, or the cartridge, which in
/// turn can arbitrarily map the memory access to its contents.
pub trait Bus {
    fn load(&self, addr: Address) -> u8;

    fn store(&mut self, addr: Address, value: u8);
}

/// It can be useful to treat the 16-bit address space as an array for testing.
impl Bus for [u8; 0x10000] {
    fn load(&self, addr: Address) -> u8 {
        self[addr.as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self[addr.as_usize()] = value;
    }
}
