pub use address::Address;

mod address;

use crate::io::IoRegister;
use crate::ppu::{Ppu, PpuBus};

const RAM_SIZE: usize = 2048;
const RAM_ADDR_BITS: u8 = 11;

const PPU_REG_START: Address = Address(0x2000);
const IO_REG_START: Address = Address(0x4000);
const CART_SPACE_START: Address = Address(0x4020);

/// Trait representing the CPU's address bus. The actual destination of loads
/// and stores are mapped by hardware to several possible locations, including
/// the NES's RAM, the PPU, various IO registers, or the cartridge, which in
/// turn can arbitrarily map the memory access to its contents. Note that even
/// loads take `&mut self` since loading from a device could potentially change
/// its internal state.
pub trait Bus {
    fn load(&mut self, addr: Address) -> u8;

    fn store(&mut self, addr: Address, value: u8);

    fn load_range(&mut self, start: Address, output: &mut [u8]) {
        for i in 0..output.len() {
            output[i] = self.load(start + i);
        }
    }

    fn store_range(&mut self, start: Address, input: &[u8]) {
        for i in 0..input.len() {
            self.store(start + i, input[i]);
        }
    }
}

/// It can be useful to treat the 16-bit address space as an array for testing.
impl Bus for [u8; 0x10000] {
    fn load(&mut self, addr: Address) -> u8 {
        self[addr.as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self[addr.as_usize()] = value;
    }
}

/// The NES's actual RAM. Since it is much smaller than the address space,
/// out-of-range addresses are aliased by dropping the high order bits. This
/// causes the contents of RAM to be mirrored throughout its portion of the
/// address space, which is how the NES hardware behaves in practice.
pub struct Ram([u8; RAM_SIZE]);

impl Ram {
    pub fn new() -> Self {
        Ram([0; RAM_SIZE])
    }
}

impl Bus for Ram {
    fn load(&mut self, addr: Address) -> u8 {
        self.0[addr.alias(RAM_ADDR_BITS).as_usize()]
    }

    fn store(&mut self, addr: Address, value: u8) {
        self.0[addr.alias(RAM_ADDR_BITS).as_usize()] = value;
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
/// Mirroring is accomplished by masking off the required number of high order
/// bits of the requested address; this emulates the incomplete decoding of
/// address lines by the NES hardware.
///
/// This struct is intended to be used to temporarily borrow all of the NES's
/// components to pass to the CPU. The details of the actual memory mapping
/// are abstracted away; from the CPU's perspective, this is just one big
/// address space.
pub struct Memory<'a, M, P> {
    ram: &'a mut Ram,
    ppu: &'a mut Ppu<P>,
    mapper: &'a mut M,
}

impl<'a, M: Bus, P: PpuBus> Memory<'a, M, P> {
    pub fn new(ram: &'a mut Ram, ppu: &'a mut Ppu<P>, mapper: &'a mut M) -> Self {
        Self { ram, ppu, mapper }
    }

    pub fn read_io_register(&mut self, addr: Address) -> u8 {
        let reg = IoRegister::from(addr);

        use IoRegister::*;
        let value = match reg {
            Sq1Vol => 0,
            Sq1Sweep => 0,
            Sq1Lo => 0,
            Sq1Hi => 0,
            Sq2Vol => 0,
            Sq2Sweep => 0,
            Sq2Lo => 0,
            Sq2Hi => 0,
            TriLinear => 0,
            TriLo => 0,
            TriHi => 0,
            NoiseVol => 0,
            NoiseLo => 0,
            NoiseHi => 0,
            DmcFreq => 0,
            DmcRaw => 0,
            DmcStart => 0,
            DmcLen => 0,
            OamDma => 0,
            SndChn => 0,
            Joy1 => 0,
            Joy2 => 0,
        };
        log::debug!("Read from IO register {} ({}): {:#X}", reg, addr, value);

        value
    }

    pub fn write_io_register(&mut self, addr: Address, value: u8) {
        let reg = IoRegister::from(addr);

        log::debug!("Write to IO register {} ({}): {:#X}", reg, addr, value);

        use IoRegister::*;
        match reg {
            Sq1Vol => {}
            Sq1Sweep => {}
            Sq1Lo => {}
            Sq1Hi => {}
            Sq2Vol => {}
            Sq2Sweep => {}
            Sq2Lo => {}
            Sq2Hi => {}
            TriLinear => {}
            TriLo => {}
            TriHi => {}
            NoiseVol => {}
            NoiseLo => {}
            NoiseHi => {}
            DmcFreq => {}
            DmcRaw => {}
            DmcStart => {}
            DmcLen => {}
            OamDma => {
                let mut oam_data = [0u8; 256];
                let start = Address::from([0, value]);
                log::debug!("Loading OAM data from address {}", &start);
                self.load_range(start, &mut oam_data);
                dbg!(&oam_data);
                self.ppu.oam_dma(oam_data);
            }
            SndChn => {}
            Joy1 => {}
            Joy2 => {}
        };
    }
}

impl<'a, M: Bus, P: PpuBus> Bus for Memory<'a, M, P> {
    fn load(&mut self, addr: Address) -> u8 {
        if addr < PPU_REG_START {
            // Read from system RAM.
            self.ram.load(addr)
        } else if addr < IO_REG_START {
            // Read from a memory-mapped PPU register.
            self.ppu.load(addr)
        } else if addr < CART_SPACE_START {
            self.read_io_register(addr)
        } else {
            // Read from the cartridge (via the mapper).
            self.mapper.load(addr)
        }
    }

    fn store(&mut self, addr: Address, value: u8) {
        if addr < PPU_REG_START {
            // Write to system RAM.
            self.ram.store(addr, value);
        } else if addr < IO_REG_START {
            // Write to a memory-mapped PPU register.
            self.ppu.store(addr, value);
        } else if addr < CART_SPACE_START {
            self.write_io_register(addr, value);
        } else {
            // Write to the cartidge memory (via the mapper).
            self.mapper.store(addr, value);
        }
    }
}
