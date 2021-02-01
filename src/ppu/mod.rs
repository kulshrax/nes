use crate::mem::{Address, Bus};

const VRAM_SIZE: usize = 2048;

// Since there are only 8 PPU registers, only the last 3 address bits are used
// to determine which register to select.
const PPU_REG_ADDR_BITS: u8 = 3;

#[derive(Default)]
struct Registers {
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    oam_data: u8,
    scroll: u8,
    addr: u8,
    data: u8,
}

impl Registers {
    fn new() -> Self {
        Default::default()
    }
}

/// The CPU can interact with the PPU via its registers, which are mapped into
/// the CPU's address space. Only the last 3 bits of the address are decoded,
/// meaning that the registers are mirrored every 8-bits.
impl Bus for Registers {
    fn load(&self, addr: Address) -> u8 {
        match addr.alias(PPU_REG_ADDR_BITS).as_usize() {
            0 => self.ctrl,
            1 => self.mask,
            2 => self.status,
            3 => self.oam_addr,
            4 => self.oam_data,
            5 => self.scroll,
            6 => self.addr,
            7 => self.data,
            _ => unreachable!(),
        }
    }

    fn store(&mut self, addr: Address, value: u8) {
        let reg = match addr.alias(PPU_REG_ADDR_BITS).as_usize() {
            0 => &mut self.ctrl,
            1 => &mut self.mask,
            2 => &mut self.status,
            3 => &mut self.oam_addr,
            4 => &mut self.oam_data,
            5 => &mut self.scroll,
            6 => &mut self.addr,
            7 => &mut self.data,
            _ => unreachable!(),
        };
        *reg = value;
    }
}

/// Trait representing the PPU's address bus, which is used to access the PPU's
/// address space (separate from the CPU addres space). PPU memory accesses can
/// be arbitrarily remapped by the cartridge, which is why a reference to the
/// PPU's VRAM is passed into these methods (so that the mapper can choose to
/// map a read or write to VRAM).
pub trait PpuBus {
    fn load(&self, vram: &Vram, addr: Address) -> u8;

    fn store(&mut self, vram: &mut Vram, addr: Address, value: u8);
}

pub struct Ppu {
    registers: Registers,
    vram: Vram,
    oam: [u8; 256],
    palette: [u8; 32],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            vram: Vram::new(),
            oam: [0; 256],
            palette: [0; 32],
        }
    }
}

impl Bus for Ppu {
    fn load(&self, addr: Address) -> u8 {
        self.registers.load(addr)
    }

    fn store(&mut self, addr: Address, value: u8) {
        self.registers.store(addr, value);
    }
}

/// The PPU has its own dedicated VRAM separate from the CPU, primarily used to
/// store the nametables. Note that although the NES logically has 4 nametables,
/// the VRAM is only large enough to store 2 of them. Games can work around this
/// by mapping the remainder of the VRAM address range to the cartridge itself
/// (which presumably has additional RAM chips). Otherwise, the contents of VRAM
/// are mirrored to fill up the available address range for nametables.
pub struct Vram([u8; VRAM_SIZE]);

impl Vram {
    fn new() -> Self {
        Vram([0; VRAM_SIZE])
    }

    pub fn inner(&self) -> &[u8; VRAM_SIZE] {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut [u8; VRAM_SIZE] {
        &mut self.0
    }
}
