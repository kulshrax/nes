struct Ppu {}

impl Ppu {
    fn new() -> Self {
        Self {}
    }
}


struct Registers {
    ctrl: u8,
    mask: u8
    status: u8,
    oam_addr: u8,
    oam_data: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    oam_dma: u8,
}