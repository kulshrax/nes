pub use address::Address;

mod address;

use crate::ppu::Ppu;
use crate::rom::Rom;

const RAM_SIZE: usize = 2048;
const RAM_ADDR_BITS: u8 = 11;

const PPU_REG_START: Address = Address(0x2000);
const IO_REG_START: Address = Address(0x4000);
const CART_SPACE_START: Address = Address(0x4020);

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

/// Memory map of the NES's CPU address space, laid out as folows:
///
///   0x0000 - 0x07FF: RAM (2kB)
///   0x0800 - 0x1FFF: Mirrors of RAM (repeated 3 times)
///   0x2000 - 0x2007: Memory mapped PPU registers (8 total)
///   0x2008 - 0x3FFF: Mirrors of PPU registers (every 8 bytes)
///   0x4000 - 0x4017: IO registers (for APU, controllers, etc.)
///   0x4018 - 0x401F: Test mode registers (disabled on production NES)
///   0x4020 - 0xFFFF: Cartridge address space (PRG ROM, PRG RAM, mappers)
///
pub struct Memory {
    ram: [u8; RAM_SIZE],
    ppu: Ppu,
    rom: Rom,
}

impl Memory {
    pub fn new(rom: Rom) -> Self {
        Self {
            ram: [0; RAM_SIZE],
            ppu: Ppu::new(),
            rom: rom,
        }
    }
}

impl Bus for Memory {
    fn load(&self, addr: Address) -> u8 {
        if addr < PPU_REG_START {
            // Read from RAM.
            self.ram[addr.alias(RAM_ADDR_BITS).as_usize()]
        } else if addr < IO_REG_START {
            // Read from a PPU register.
            self.ppu.load(addr)
        } else if addr < CART_SPACE_START {
            // Read from an IO register.
            unimplemented!()
        } else {
            // Read from the cartridge via its mapper.
            unimplemented!()
        }
    }

    fn store(&mut self, addr: Address, value: u8) {
        if addr < PPU_REG_START {
            // Write to RAM.
            self.ram[addr.alias(RAM_ADDR_BITS).as_usize()] = value;
        } else if addr < IO_REG_START {
            // Write to a PPU register.
            self.ppu.store(addr, value);
        } else if addr < CART_SPACE_START {
            // Write to an IO register.
            unimplemented!()
        } else {
            // Write to the cartridge via its mapper.
            unimplemented!()
        }
    }
}
