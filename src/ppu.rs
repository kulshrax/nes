use pixels::Pixels;
use winit::window::Window;

use crate::mem::{Address, Bus};

pub const VRAM_SIZE: usize = 2048;

pub const NAMETABLE_BASE_ADDR: Address = Address(0x2000);

pub const PALETTE_BASE_ADDR: Address = Address(0x3F00);
pub const PALETTE_ADDR_BITS: u8 = 5;

// Since there are only 8 PPU registers, only the last 3 address bits are used
// to determine which register to select.
const PPU_REG_ADDR_BITS: u8 = 3;

enum PpuRegister {
    Ctrl,
    Mask,
    Status,
    OamAddr,
    OamData,
    Scroll,
    Addr,
    Data,
}

impl From<Address> for PpuRegister {
    fn from(addr: Address) -> Self {
        use PpuRegister::*;
        match addr.alias(PPU_REG_ADDR_BITS).as_usize() {
            0 => Ctrl,
            1 => Mask,
            2 => Status,
            3 => OamAddr,
            4 => OamData,
            5 => Scroll,
            6 => Addr,
            7 => Data,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Registers {
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    scroll: [Option<u8>; 2],
    addr: [Option<u8>; 2],

    // Contains the most recently written or read value from any register. This
    // is used to mimic the behavior of the data bus between the NES's CPU and
    // PPU, which retains the value of the most recent read or write. Attempts
    // to read from a write-only register will return this retained value.
    most_recent_value: u8,
}

/// Trait representing the PPU's address bus, which is used to access the PPU's
/// address space (separate from the CPU addres space). PPU memory accesses can
/// be arbitrarily remapped by the cartridge, which is why a reference to the
/// PPU's VRAM is passed into these methods (so that the mapper can choose to
/// map a read or write to VRAM).
pub trait PpuBus {
    fn ppu_load(&mut self, vram: &Vram, addr: Address) -> u8;

    fn ppu_store(&mut self, vram: &mut Vram, addr: Address, value: u8);
}

pub struct Ppu<M> {
    registers: Registers,
    vram: Vram,
    oam: [u8; 256],
    palette: [u8; 32],
    mapper: M,
}

impl<M> Ppu<M> {
    pub fn with_mapper(mapper: M) -> Self {
        Self {
            registers: Registers::default(),
            vram: Vram::new(),
            oam: [0; 256],
            palette: [0; 32],
            mapper,
        }
    }

    pub fn tick(&self, _frame: &mut Pixels<Window>) {
        unimplemented!()
    }
}

/// The CPU can interact with the PPU via its registers, which are mapped into
/// the CPU's address space. Only the last 3 bits of the address are decoded,
/// meaning that the registers are mirrored every 8-bits.
impl<M: PpuBus> Bus for Ppu<M> {
    fn load(&mut self, addr: Address) -> u8 {
        use PpuRegister::*;

        let value = match addr.into() {
            Status => {
                // Reading the status register clears the address and scroll
                // registers.
                self.registers.scroll = [None, None];
                self.registers.addr = [None, None];
                self.registers.status
            }
            OamData => self.oam[self.registers.oam_addr as usize],
            Data => {
                let addr = read_ppuaddr(&self.registers.addr);
                if addr < PALETTE_BASE_ADDR {
                    // Read from PPU address space via mapper.
                    self.mapper.ppu_load(&self.vram, addr)
                } else {
                    let i = addr.alias(PALETTE_ADDR_BITS).as_usize();
                    self.palette[i]
                }
            }
            // All other registers are write-only, and therefore attempts to
            // read their values will just return whatever value is presently
            // on the data bus (i.e., whatever value was most recently read or
            // written).
            _ => self.registers.most_recent_value,
        };

        self.registers.most_recent_value = value;

        value
    }

    fn store(&mut self, addr: Address, value: u8) {
        use PpuRegister::*;

        self.registers.most_recent_value = value;
        match addr.into() {
            Ctrl => self.registers.ctrl = value,
            Mask => self.registers.mask = value,
            Status => {} // Status register is read-only.
            OamAddr => self.registers.oam_addr = value,
            OamData => self.oam[self.registers.oam_addr as usize] = value,
            Scroll => double_write(&mut self.registers.scroll, value),
            Addr => double_write(&mut self.registers.addr, value),
            Data => {
                let addr = read_ppuaddr(&self.registers.addr);
                if addr < PALETTE_BASE_ADDR {
                    // Write to PPU address space via mapper.
                    self.mapper.ppu_store(&mut self.vram, addr, value);
                } else {
                    let i = addr.alias(PALETTE_ADDR_BITS).as_usize();
                    self.palette[i] = value;
                }
            }
        };
    }
}

/// The PPU has its own dedicated VRAM separate from the CPU, primarily used to
/// store the nametables. Note that although the NES logically has 4 nametables,
/// the VRAM is only large enough to store 2 of them. Games can work around this
/// by mapping the remainder of the VRAM address range to the cartridge itself
/// (which presumably has additional RAM chips). Otherwise, the contents of VRAM
/// are mirrored to fill up the available address range for nametables.
pub struct Vram(pub [u8; VRAM_SIZE]);

impl Vram {
    fn new() -> Self {
        Vram([0; VRAM_SIZE])
    }
}

// Emulate the behavior of the PPUADDR and PPUSCROLL registers, which require
// the CPU to write 2 bytes. Since each register is only mapped to a single
// byte of the CPU's address space, the CPU must perform 2 writes in succession.
fn double_write(reg: &mut [Option<u8>; 2], value: u8) {
    *reg = match *reg {
        [None, None] => [Some(value), None],
        [Some(first), None] => [Some(first), Some(value)],
        _ => return,
    };
}

/// Intepret the contents of the PPUADDR register.
fn read_ppuaddr(addr: &[Option<u8>; 2]) -> Address {
    let high = addr[0].unwrap_or(0);
    let low = addr[1].unwrap_or(0);
    Address::from([low, high])
}
