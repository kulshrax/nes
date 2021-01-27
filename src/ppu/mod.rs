pub enum PpuRegister {
    Ctrl,
    Mask,
    Status,
    Scroll,
    Addr,
    Data,
    OamAddr,
    OamData,
    OamDma,
}

#[derive(Default)]
struct Registers {
    ctrl: u8,
    mask: u8,
    status: u8,
    scroll: u8,
    addr: u8,
    data: u8,
    oam_addr: u8,
    oam_data: u8,
    oam_dma: u8,
}

impl Registers {
    fn get(&self, reg: PpuRegister) -> u8 {
        use PpuRegister::*;
        match reg {
            Ctrl => self.ctrl,
            Mask => self.mask,
            Status => self.status, 
            Scroll => self.scroll,
            Addr => self.addr,
            Data => self.data,
            OamAddr => self.oam_addr,
            OamData => self.oam_data,
            OamDma => self.oam_dma,
        }
    } 

    fn get_mut(&mut self, reg: PpuRegister) -> &mut u8 {
        use PpuRegister::*;
        match reg {
            Ctrl => &mut self.ctrl,
            Mask => &mut self.mask,
            Status => &mut self.status, 
            Scroll => &mut self.scroll,
            Addr => &mut self.addr,
            Data => &mut self.data,
            OamAddr => &mut self.oam_addr,
            OamData => &mut self.oam_data,
            OamDma => &mut self.oam_dma,
        }
    } 
}

pub struct Ppu {
    registers: Registers,
    vram: [u8; 2048],
    oam: [u8; 256],
    palette: [u8; 32],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            registers: Registers::default(),
            vram: [0; 2048],
            oam: [0; 256],
            palette: [0; 32],
        }
    }

    pub fn read(&self, reg: PpuRegister) -> u8 {
        self.registers.get(reg)
    } 

    pub fn write(&mut self, reg: PpuRegister, value: u8) {
        *self.registers.get_mut(reg) = value;
    }
}