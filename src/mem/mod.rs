pub use address::Address;
pub use bus::Bus;

mod address;
mod bus;

use crate::ppu::Ppu;
use crate::rom::Rom;

const RAM_SIZE: usize = 2048;
const PPU_REG_START: Address = Address(0x2000);
const IO_REG_START: Address = Address(0x4000);
const CART_SPACE_START: Address = Address(0x4020);

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
            self.ram[addr.as_usize()]
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
            self.ram[addr.as_usize()] = value;
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
