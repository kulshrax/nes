pub use address::Address;

mod address;

use crate::ppu::{Ppu, PpuBus};

const RAM_SIZE: usize = 2048;
const RAM_ADDR_BITS: u8 = 11;

const PPU_REG_START: Address = Address(0x2000);
const IO_REG_START: Address = Address(0x4000);
const CART_SPACE_START: Address = Address(0x4020);

const SQ1_VOL: Address = Address(0x4000);
const SQ1_SWEEP: Address = Address(0x4001);
const SQ1_LO: Address = Address(0x4002);
const SQ1_HI: Address = Address(0x4003);
const SQ2_VOL: Address = Address(0x4004);
const SQ2_SWEEP: Address = Address(0x4005);
const SQ2_LO: Address = Address(0x4006);
const SQ2_HI: Address = Address(0x4007);
const TRI_LINEAR: Address = Address(0x4008);
const TRI_LO: Address = Address(0x400A);
const TRI_HI: Address = Address(0x400B);
const NOISE_VOL: Address = Address(0x400C);
const NOISE_LO: Address = Address(0x400E);
const NOISE_HI: Address = Address(0x400F);
const DMC_FREQ: Address = Address(0x4010);
const DMC_RAW: Address = Address(0x4011);
const DMC_START: Address = Address(0x4012);
const DMC_LEN: Address = Address(0x4013);
const OAMDMA: Address = Address(0x4014);
const SND_CHN: Address = Address(0x4015);
const JOY1: Address = Address(0x4016);
const JOY2: Address = Address(0x4017);

fn io_register_name(addr: Address) -> &'static str {
    match addr {
        SQ1_VOL => "SQ1_VOL",
        SQ1_SWEEP => "SQ1_SWEEP",
        SQ1_LO => "SQ1_LO",
        SQ1_HI => "SQ1_HI",
        SQ2_VOL => "SQ2_VOL",
        SQ2_SWEEP => "SQ2_SWEEP",
        SQ2_LO => "SQ2_LO",
        SQ2_HI => "SQ2_HI",
        TRI_LINEAR => "TRI_LINEAR",
        TRI_LO => "TRI_LO",
        TRI_HI => "TRI_HI",
        NOISE_VOL => "NOISE_VOL",
        NOISE_LO => "NOISE_LO",
        NOISE_HI => "NOISE_HI",
        DMC_FREQ => "DMC_FREQ",
        DMC_RAW => "DMC_RAW",
        DMC_START => "DMC_START",
        DMC_LEN => "DMC_LEN",
        OAMDMA => "OAMDMA",
        SND_CHN => "SND_CHN",
        JOY1 => "JOY1",
        JOY2 => "JOY2",
        _ => "invalid",
    }
}

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
        // Read from an IO register.
        log::error!(
            "read from IO register {} ({})",
            io_register_name(addr),
            addr
        );
        match addr {
            SQ1_VOL => {}
            SQ1_SWEEP => {}
            SQ1_LO => {}
            SQ1_HI => {}
            SQ2_VOL => {}
            SQ2_SWEEP => {}
            SQ2_LO => {}
            SQ2_HI => {}
            TRI_LINEAR => {}
            TRI_LO => {}
            TRI_HI => {}
            NOISE_VOL => {}
            NOISE_LO => {}
            NOISE_HI => {}
            DMC_FREQ => {}
            DMC_RAW => {}
            DMC_START => {}
            DMC_LEN => {}
            OAMDMA => {}
            SND_CHN => {}
            JOY1 => {}
            JOY2 => {}
            _ => {}
        };
        0
    }

    pub fn write_io_register(&mut self, addr: Address, value: u8) {
        log::debug!(
            "write to IO register {} ({}): {}",
            io_register_name(addr),
            addr,
            value
        );
        match addr {
            SQ1_VOL => {}
            SQ1_SWEEP => {}
            SQ1_LO => {}
            SQ1_HI => {}
            SQ2_VOL => {}
            SQ2_SWEEP => {}
            SQ2_LO => {}
            SQ2_HI => {}
            TRI_LINEAR => {}
            TRI_LO => {}
            TRI_HI => {}
            NOISE_VOL => {}
            NOISE_LO => {}
            NOISE_HI => {}
            DMC_FREQ => {}
            DMC_RAW => {}
            DMC_START => {}
            DMC_LEN => {}
            OAMDMA => {
                let mut oam_data = [0u8; 256];
                let start = Address::from([0, value]);
                log::debug!("loading OAM data from address {}", &start);
                self.load_range(start, &mut oam_data);
                self.ppu.oam_dma(oam_data);
            }
            SND_CHN => {}
            JOY1 => {}
            JOY2 => {}
            _ => {}
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
