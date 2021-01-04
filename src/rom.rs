use std::{fs::File, io::prelude::*, path::Path};

use anyhow::Result;
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
    pub prg: Vec<u8>,
    pub chr: Vec<u8>,
}

impl Rom {
    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let mut buf = Vec::new();
        let mut f = File::open(path.as_ref())?;
        f.read_to_end(&mut buf)?;

        // XXX: Nom's error types are a bit complicated and contain references.
        // Converting to an owned representation (even just converting to an
        // owned string) is non-trivial. Rather than dealing with this, just
        // panic if we can't parse the ROM (since we'd want to exit the program
        // anyway if we can't load the input).
        let (_, (prg, chr)) = parse_rom(&buf).expect("Failed to parse ROM file");

        Ok(Rom {
            prg: prg.to_vec(),
            chr: chr.to_vec(),
        })
    }
}

/// Identify and return slices containing the ROM file's PRG and CHR banks.
fn parse_rom(bytes: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    // Initial 4 byte magic sequence.
    let (bytes, _) = tag(b"NES\x1A")(bytes.as_ref())?;

    // Number of PRG (program) and CHR (character) banks.
    let (bytes, prg_banks) = le_u8(bytes)?;
    let (bytes, chr_banks) = le_u8(bytes)?;

    // Other fields we don't care about yet.
    let (bytes, _) = take(10usize)(bytes)?;

    // Actual PRG and CHR bank data.
    let (bytes, prg_data) = take(prg_banks as usize * PRG_BANK_SIZE)(bytes)?;
    let (bytes, chr_data) = take(chr_banks as usize * CHR_BANK_SIZE)(bytes)?;

    Ok((bytes, (prg_data, chr_data)))
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
