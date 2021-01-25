pub use address::Address;
pub use bus::Bus;

mod address;
mod bus;

use crate::rom::Rom;

const RAM_SIZE: usize = 2048;

const PPU_REG_START: Address = Address(0x2000);
const PPU_REG_SIZE: usize = 8;

const IO_REG_START: Address = Address(0x4000);
const IO_REG_SIZE: usize = 24;

const TEST_MODE_START: Address = Address(0x4018);

const CART_SPACE_START: Address = Address(0x4020);
const CART_SPACE_SIZE: usize = 1;

/// Struct representing the NES's CPU memory bus.
///
/// The NES's CPU address space is laid out as folows:
///   0x0000 - 0x7FFF: RAM (2kB)
///   0x0800 - 0x1FFF: Mirrors of RAM (repeated 3 times)
///   0x2000 - 0x2007: Memory mapped PPU registers (8 total)
///   0x2008 - 0x3FFF: Mirrors of PPU registers (every 8 bytes)
///   0x4000 - 0x4017: IO registers (for APU, controllers, etc.)
///   0x4018 - 0x401F: Test mode registers (disabled on production NES)
///   0x4020 - 0xFFFF: Cartridge address space (PRG ROM, PRG RAM, mappers)
///
pub struct Memory {
    ram: [u8; RAM_SIZE],
}

// impl From<Address> for Index {
//     fn from(addr: Address) -> Self {
//         if addr < PPU_REG_START {
//             Ram(addr % RAM_SIZE)
//         } else if addr < IO_REG_START {
//             Ppu(addr % PPU_REG_SIZE)
//         } else if addr < TEST_MODE_START {
//             Io(addr % IO_REG_SIZE)
//         } else if addr < CART_SPACE_START {
//             panic!("Address {} refers to unmapped memory", addr)
//         } else {
//             Cartridge(addr - CART_SPACE_START)
//         }
//     }
// }

impl Memory {
    pub fn new() -> Self {
        Self { ram: [0; RAM_SIZE] }
    }
}

impl Bus for Memory {
    fn load(&self, addr: Address) -> u8 {
        unimplemented!()
    }

    fn store(&mut self, addr: Address, value: u8) {
        unimplemented!()
    }
}
