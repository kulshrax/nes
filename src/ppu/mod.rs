use crate::mem::{Address, Bus};

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

pub struct Ppu {
    registers: [u8; 8],
    vram: [u8; 2048],
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
