use super::address::Address;

/// Trait representing an address bus, which can be used by the CPU to read and
/// write values. Note that the actual destination of these writes can be mapped
/// by hardware mappers on the cartridge, which is why this needs to be a trait.
pub trait Bus {
    fn load(&self, addr: Address) -> u8;

    fn store(&mut self, addr: Address, value: u8);
}

impl Bus for [u8; 0x10000] {
    fn load(&self, addr: Address) -> u8 {
        self[addr.as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self[addr.as_usize()] = value;
    }
}
