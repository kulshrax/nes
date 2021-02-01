use crate::mem::{Address, Bus};

pub const VRAM_SIZE: usize = 2048;

// Since there are only 8 PPU registers, only the last 3 address bits are used
// to determine which register to select.
const PPU_REG_ADDR_BITS: u8 = 3;

#[repr(usize)]
pub enum PpuRegister {
    Ctrl = 0,
    Mask = 1,
    Status = 2,
    OamAddr = 3,
    OamData = 4,
    Scroll = 5,
    Addr = 6,
    Data = 7,
}

/// Trait representing the PPU's address bus, which is used to access the PPU's
/// address space (separate from the CPU addres space). PPU memory accesses can
/// be arbitrarily remapped by the cartridge, which is why a reference to the
/// PPU's VRAM is passed into these methods (so that the mapper can choose to
/// map a read or write to VRAM).
pub trait PpuBus {
    fn load(&self, vram: &[u8; VRAM_SIZE], addr: Address) -> u8;

    fn store(&mut self, vram: &mut [u8; VRAM_SIZE], addr: Address, value: u8);
}

pub struct Ppu {
    registers: [u8; 8],
    vram: [u8; VRAM_SIZE],
    oam: [u8; 256],
    palette: [u8; 32],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            registers: [0; 8],
            vram: [0; 2048],
            oam: [0; 256],
            palette: [0; 32],
        }
    }

    pub fn get_reg(&self, reg: PpuRegister) -> u8 {
        self.registers[reg as usize]
    }

    pub fn set_reg(&mut self, reg: PpuRegister, value: u8) {
        self.registers[reg as usize] = value;
    }
}

impl Bus for Ppu {
    fn load(&self, addr: Address) -> u8 {
        self.registers[addr.alias(PPU_REG_ADDR_BITS).as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self.registers[addr.alias(PPU_REG_ADDR_BITS).as_usize()] = value;
    }
}
