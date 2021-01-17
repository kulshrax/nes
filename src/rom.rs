use std::{fs::File, io::prelude::*, path::Path};

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::{tag, take},
    number::complete::le_u8,
    IResult,
};

const PRG_BANK_SIZE: usize = 16384; // 16 KiB
const CHR_BANK_SIZE: usize = 8192; // 8 KiB

/// The contents of an iNES-format ROM file.
#[derive(Debug)]
pub struct Rom {
    // ROM sizes are specified as multiples of the appropriate bank size.
    pub prg_size: usize,
    pub chr_size: usize,

    // Contents of cartridge ROM chips.
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
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
    let (bytes, prg_size) = le_u8(bytes)?;
    let prg_size = prg_size as usize;

    let (bytes, chr_size) = le_u8(bytes)?;
    let chr_size = chr_size as usize;

    // Other fields we don't care about yet.
    let (bytes, _) = take(10usize)(bytes)?;

    // Actual PRG and CHR bank data.
    let (bytes, prg_rom) = take(prg_size * PRG_BANK_SIZE)(bytes)?;
    let (bytes, chr_rom) = take(chr_size * CHR_BANK_SIZE)(bytes)?;

    let rom = Rom {
        prg_size,
        chr_size,
        prg_rom: prg_rom.to_vec(),
        chr_rom: chr_rom.to_vec(),
    };

    Ok((bytes, rom))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load() -> Result<()> {
        let _ = Rom::load("test/nestest.nes")?;
        Ok(())
    }
}
