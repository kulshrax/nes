pub struct Ppu {
    ram: [u8; 2048],
    oam: [u8; 256],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ram: [0u8; 2048],
            oam: [0u8; 256],
        }
    }
}