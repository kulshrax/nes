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
    let num_prg_banks = num_prg_banks as usize;

    let (bytes, num_chr_banks) = le_u8(bytes)?;
    let num_chr_banks = num_chr_banks as usize;

    // Other fields we don't care about yet.
    let (bytes, _) = take(10usize)(bytes)?;

    // Actual PRG and CHR bank data.
    let (bytes, prg) = take(num_prg_banks * PRG_BANK_SIZE)(bytes)?;
    let (bytes, chr) = take(num_chr_banks * CHR_BANK_SIZE)(bytes)?;

    let rom = Rom {
        prg: prg.to_vec(),
        chr: chr.to_vec(),
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
