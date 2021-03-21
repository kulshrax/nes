use std::{fs::File, io::prelude::*, path::Path};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take},
    number::complete::{le_u16, le_u8},
    IResult,
};

const PRG_BANK_SIZE: usize = 16384; // 16 KiB
const CHR_BANK_SIZE: usize = 8192; // 8 KiB

#[derive(Debug)]
pub struct Header {
    pub num_prg_banks: u8,
    pub num_chr_banks: u8,
    pub num_prg_ram_banks: u8,
    pub mirroring: Mirroring,
    pub mapper: u8,
    pub has_battery: bool,
    pub has_trainer: bool,
    pub is_ines_v2: bool,
}

impl Header {
    fn new(num_prg_banks: u8, num_chr_banks: u8, num_prg_ram_banks: u8, flags: u16) -> Self {
        let mirroring = {
            let b0 = flags & 0x01 > 0;
            let b3 = flags & 0x08 > 0;
            match (b0, b3) {
                (_, true) => Mirroring::None,
                (true, false) => Mirroring::Vertical,
                (false, false) => Mirroring::Horizonal,
            }
        };

        let has_battery = flags & 0x02 > 0;
        let has_trainer = flags & 0x04 > 0;

        let mapper = {
            // The lower 4 bits and upper 4 bits of the 8-bit mapper number are
            // stored as the top 4 bits of bytes 6 and 7 respectively.
            let low = (flags & (0x0F << 4)) >> 4;
            let high = (flags & (0x0F << 12)) >> 8;
            (low | high) as u8
        };

        let is_ines_v2 = flags & (0x03 << 10) >> 10 == 2;

        Self {
            num_prg_banks,
            num_chr_banks,
            num_prg_ram_banks,
            mirroring,
            mapper,
            has_battery,
            has_trainer,
            is_ines_v2,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Mirroring {
    Horizonal,
    Vertical,
    None,
}

/// The contents of an iNES-format ROM file.
#[derive(Debug)]
pub struct Rom {
    pub header: Header,

    // Program (PRG) ROM banks.
    pub prg: Vec<u8>,

    // Character (CHR) ROM banks.
    pub chr: Vec<u8>,
}

impl Rom {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut buf = Vec::new();
        let mut f = File::open(path.as_ref())?;
        f.read_to_end(&mut buf)?;

        let (_, rom) = parse_rom(&buf).map_err(|_| anyhow!("Failed to parse ROM file"))?;

        Ok(rom)
    }
}

/// Parse a the content of an iNES-format ROM file.
fn parse_rom(bytes: &[u8]) -> IResult<&[u8], Rom> {
    // Initial 4 byte magic sequence.
    let (bytes, _) = tag(b"NES\x1A")(bytes.as_ref())?;

    // Number of PRG (program) and CHR (character) ROM banks.
    let (bytes, num_prg_banks) = le_u8(bytes)?;
    let (bytes, num_chr_banks) = le_u8(bytes)?;

    // Get flags from bytes 6 and 7.
    let (bytes, flags) = le_u16(bytes)?;

    // Byte 8 contains an optional PRG RAM size.
    let (bytes, num_prg_ram_banks) = le_u8(bytes)?;

    // Ignore flag bytes 9 and 10 since these are rarely used iNES format
    // extensions. Bytes 11-15 are unused padding.
    let (bytes, _) = take(7usize)(bytes)?;

    let header = Header::new(num_prg_banks, num_chr_banks, num_prg_ram_banks, flags);

    // If a trainer is present, skip over it.
    let bytes = if header.has_trainer {
        let (bytes, _) = take(512usize)(bytes)?;
        bytes
    } else {
        bytes
    };

    // Actual PRG and CHR bank data.
    let (bytes, prg) = take(num_prg_banks as usize * PRG_BANK_SIZE)(bytes)?;
    let (bytes, chr) = take(num_chr_banks as usize * CHR_BANK_SIZE)(bytes)?;

    let rom = Rom {
        header,
        prg: prg.to_vec(),
        chr: chr.to_vec(),
    };

    Ok((bytes, rom))
}
