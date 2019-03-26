use std::{
    fs::File,
    io::{self, prelude::*},
    path::Path,
};

pub struct Rom(pub Vec<u8>);

impl Rom {
    pub fn load(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut buf = Vec::new();
        let mut f = File::open(path.as_ref())?;
        f.read_to_end(&mut buf)?;
        Ok(Self(buf))
    }
}
