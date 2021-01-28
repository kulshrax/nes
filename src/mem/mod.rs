pub use address::Address;
pub use bus::Bus;

mod address;
mod bus;
mod mapper;

use crate::ppu::Ppu;
use crate::rom::Rom;

const RAM_SIZE: usize = 2048;
const RAM_ADDR_BITS: u8 = 11;

const PPU_REG_START: Address = Address(0x2000);
const PPU_REG_SIZE: usize = 8;
const PPU_REG_BITS: u8 = 3;

const IO_REG_START: Address = Address(0x4000);
const IO_REG_SIZE: usize = 32;
const IO_REG_BITS: usize = 5;

const CART_SPACE_START: Address = Address(0x4020);
const CART_SPACE_SIZE: usize = 49152; // Remainder of address space.

/// Struct representing the NES's CPU memory bus.
///
/// The NES's CPU address space is laid out as folows:
///   0x0000 - 0x07FF: RAM (2kB)
///   0x0800 - 0x1FFF: Mirrors of RAM (repeated 3 times)
///   0x2000 - 0x2007: Memory mapped PPU registers (8 total)
///   0x2008 - 0x3FFF: Mirrors of PPU registers (every 8 bytes)
///   0x4000 - 0x4017: IO registers (for APU, controllers, etc.)
///   0x4018 - 0x401F: Test mode registers (disabled on production NES)
///   0x4020 - 0xFFFF: Cartridge address space (PRG ROM, PRG RAM, mappers)
///
pub struct Memory {
    ram: Ram,
    // ppu: Ppu,
    // rom: Rom,
}

impl Memory {
    pub fn new() -> Self {
        Self { ram: Ram::new() }
    }
}

impl Bus for Memory {
    fn load(&self, addr: Address) -> u8 {
        if addr < PPU_REG_START {
            // Read from RAM.
            unimplemented!()
        } else if addr < IO_REG_START {
            // Read from PPU registers.
            unimplemented!()
        } else if addr < CART_SPACE_START {
            // Read from IO registers.
            unimplemented!()
        } else {
            // Read from cartridge.
            unimplemented!()
        }
    }

    fn store(&mut self, addr: Address, value: u8) {
        unimplemented!()
    }
}

struct Ram([u8; RAM_SIZE]);

impl Ram {
    fn new() -> Self {
        Ram([0; RAM_SIZE])
    }
}

impl Bus for Ram {
    fn load(&self, addr: Address) -> u8 {
        self.0[addr.as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self.0[addr.as_usize()] = value;
    }
}
