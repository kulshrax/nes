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
    pub prg_count: usize,
    pub chr_count: usize,
    pub prg_data: Vec<u8>,
    pub chr_data: Vec<u8>,
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

    // Number of PRG (program) and CHR (character) banks.
    let (bytes, prg_count) = le_u8(bytes)?;
    let prg_count = prg_count as usize;

    let (bytes, chr_count) = le_u8(bytes)?;
    let chr_count = chr_count as usize;

    // Other fields we don't care about yet.
    let (bytes, _) = take(10usize)(bytes)?;

    // Actual PRG and CHR bank data.
    let (bytes, prg_data) = take(prg_count * PRG_BANK_SIZE)(bytes)?;
    let (bytes, chr_data) = take(chr_count * CHR_BANK_SIZE)(bytes)?;

    let rom = Rom {
        prg_count,
        chr_count,
        prg_data: prg_data.to_vec(),
        chr_data: chr_data.to_vec(),
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
