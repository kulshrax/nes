pub use address::Address;

mod address;

pub trait Memory {
    fn load(&self, addr: Address) -> u8;

    fn store(&mut self, addr: Address, value: u8);
}

impl Memory for [u8; 0x10000] {
    fn load(&self, addr: Address) -> u8 {
        self[addr.as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self[addr.as_usize()] = value;
    }
}
