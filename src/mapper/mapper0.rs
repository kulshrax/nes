use crate::mem::{Address, Bus};
use crate::ppu::{PpuBus, Vram, NAMETABLE_0_ADDR};
use crate::rom::Rom;

use super::Mapper;

pub(super) struct Mapper0;

impl Mapper for Mapper0 {
    type CpuMapper = CpuMapper0;
    type PpuMapper = PpuMapper0;

    fn from_rom(rom: Rom) -> (CpuMapper0, PpuMapper0) {
        let Rom { prg, chr } = rom;
        (CpuMapper0::new(prg), PpuMapper0::new(chr))
    }
}

const PRG_BASE_ADDR: usize = 0x8000;
const NROM_128_SIZE: usize = 0x4000;
const NROM_256_SIZE: usize = 0x8000;

pub(super) struct CpuMapper0 {
    prg: Vec<u8>,
}

impl CpuMapper0 {
    fn new(prg: Vec<u8>) -> Self {
        // This mapper comes in 2 variants: NROM-128, which contains 16 KiB of
        // PRG ROM (128 kilobits), and NROM-256 with 32 KiB (256 kilobits).
        assert!(prg.len() == NROM_128_SIZE || prg.len() == NROM_256_SIZE);
        Self { prg }
    }
}

impl Bus for CpuMapper0 {
    fn load(&mut self, addr: Address) -> u8 {
        // NROM-256 fills the entire top half of the CPU address space.
        // NROM-128 only fills half of that space, so it is mirrored.
        let i = (addr.as_usize() - PRG_BASE_ADDR) % self.prg.len();
        self.prg[i]
    }

    fn store(&mut self, _addr: Address, _value: u8) {
        // Can't write to ROM.
    }
}

pub(super) struct PpuMapper0 {
    chr: Vec<u8>,
}

impl PpuMapper0 {
    fn new(chr: Vec<u8>) -> Self {
        // This mapper directly maps the CHR RAM into the lower portion of the
        // PPU's address space, which means it must fit exactly in the space
        // reserved for the 2 pattern tables (4 KiB each, so 8 KiB total).
        // Nametable 0 is directly after the pattern tables, so use its base
        // address to check the size.
        assert!(chr.len() == NAMETABLE_0_ADDR.as_usize());
        Self { chr }
    }
}

impl PpuBus for PpuMapper0 {
    fn ppu_load(&mut self, vram: &Vram, addr: Address) -> u8 {
        if addr < NAMETABLE_0_ADDR {
            self.chr[addr.as_usize()]
        } else {
            // TODO: Implement nametable mirroring.
            let i = addr.as_usize() - NAMETABLE_0_ADDR.as_usize();
            vram.0[i]
        }
    }

    fn ppu_store(&mut self, vram: &mut Vram, addr: Address, value: u8) {
        if addr >= NAMETABLE_0_ADDR {
            // TODO: Implement nametable mirroring.
            let i = addr.as_usize() - NAMETABLE_0_ADDR.as_usize();
            vram.0[i] = value;
        }
    }
}
