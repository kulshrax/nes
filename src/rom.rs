/// Parser for the iNES ROM format.
use std::{
    fs::File,
    io::{self, prelude::*},
    path::Path,
};

use nom::*;

const PRG_UNIT_LEN: usize = 16384; // 16 KiB
const CHR_UNIT_LEN: usize = 8192; // 8 KiB

#[derive(Debug)]
pub struct Rom {
    prg: Vec<u8>,
    chr: Vec<u8>,
}

impl Rom {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut buf = Vec::new();
        let mut f = File::open(path.as_ref())?;
        f.read_to_end(&mut buf)?;
        Self::parse(&buf)
    }

    pub fn parse(bytes: impl AsRef<[u8]>) -> io::Result<Self> {
        let (_, rom) = rom(bytes.as_ref()).expect("Failed to parse ROM");
        Ok(rom)
    }
}

named!(rom<&[u8], Rom>,
    do_parse!(
        tag!(b"NES\x1A") >>
        prg_len: le_u8 >>
        chr_len: le_u8 >>
        take!(10) >>
        prg_data: take!(prg_len as usize * PRG_UNIT_LEN) >>
        chr_data: take!(chr_len as usize * CHR_UNIT_LEN) >>
        (Rom {
            prg: prg_data.to_vec(),
            chr: chr_data.to_vec(),
        })
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> io::Result<()> {
        let _ = Rom::load("test/nestest.nes")?;
        Ok(())
    }
}
