use crate::mem::{Address, Bus};
use crate::ppu::{PpuBus, Vram, NAMETABLE_BASE_ADDR, VRAM_SIZE};
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

const CPU_BASE_ADDR: usize = 0x8000;
const NROM_128_SIZE: usize = 0x4000;
const NROM_256_SIZE: usize = 0x8000;

pub(super) struct CpuMapper0 {
    prg: Vec<u8>,
}

impl CpuMapper0 {
    fn new(prg: Vec<u8>) -> Self {
        assert!(prg.len() == NROM_128_SIZE || prg.len() == NROM_256_SIZE);
        Self { prg }
    }
}

impl Bus for CpuMapper0 {
    fn load(&mut self, addr: Address) -> u8 {
        let i = (addr.as_usize() - CPU_BASE_ADDR) % self.prg.len();
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
        assert_eq!(chr.len(), VRAM_SIZE);
        Self { chr }
    }
}

impl PpuBus for PpuMapper0 {
    fn ppu_load(&mut self, vram: &Vram, addr: Address) -> u8 {
        if addr < NAMETABLE_BASE_ADDR {
            self.chr[addr.as_usize()]
        } else {
            // TODO: Implement nametable mirroring.
            let i = addr.as_usize() - NAMETABLE_BASE_ADDR.as_usize();
            vram.0[i]
        }
    }

    fn ppu_store(&mut self, vram: &mut Vram, addr: Address, value: u8) {
        if addr >= NAMETABLE_BASE_ADDR {
            // TODO: Implement nametable mirroring.
            let i = addr.as_usize() - NAMETABLE_BASE_ADDR.as_usize();
            vram.0[i] = value;
        }
    }
}
