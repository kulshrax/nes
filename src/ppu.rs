use std::fmt;

use crate::mem::{Address, Bus};

pub const VRAM_SIZE: usize = 2048;

pub static NAMETABLES: [Address; 4] = [
    Address(0x2000),
    Address(0x2400),
    Address(0x2800),
    Address(0x2C00),
];

/// Offset from the start of a nametable where the attribute table begins.
/// The attribute table is located at the last 64 bytes of each nametable.
const ATTRIBUTE_TABLE_OFFSET: u16 = 0x3C0;

pub const PALETTE_BASE_ADDR: Address = Address(0x3F00);
pub const PALETTE_ADDR_BITS: u8 = 5;

// Since there are only 8 PPU registers, only the last 3 address bits are used
// to determine which register to select.
const PPU_REG_ADDR_BITS: u8 = 3;

pub const FRAME_WIDTH: usize = 256;
pub const FRAME_HEIGHT: usize = 240;

/// Array of 3-byte RGB color values corresponding to the colors the NES would
/// output for a given 6-bit color index. Note that in a real NES, the PPU
/// directly outputs an analog video signal, which means that there is no exact
/// mapping between NES color indexes and specific RGB values. As such, the
/// RGB values specified here are just approximations intended to best reproduce
/// what each color would have looked like when displayed by a TV set.
static NES_COLORS: &[u8] = include_bytes!("../data/FBX-Final.pal");

/// Hardcoded greyscale palette used for testing.
const GREYSCALE_PALETTE: Palette = Palette {
    background: 0x0F, // 0 0 0
    color1: 0x00,     // 84 84 84
    color2: 0x10,     // 152 150 152
    color3: 0x30,     // 236 238 236
};

// Palette locations in the PPU's address space.
const BG_COLOR: Address = Address(0x3F00);
static BG_PALETTES: [Address; 4] = [
    Address(0x3F01),
    Address(0x3F05),
    Address(0x3F09),
    Address(0x3F0D),
];
static SPRITE_PALETTES: [Address; 4] = [
    Address(0x3F11),
    Address(0x3F15),
    Address(0x3F19),
    Address(0x3F1D),
];

#[derive(Debug)]
enum PpuRegister {
    Ctrl,
    Mask,
    Status,
    OamAddr,
    OamData,
    Scroll,
    Addr,
    Data,
}

impl fmt::Display for PpuRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PpuRegister::*;
        match self {
            Ctrl => write!(f, "PPUCTRL"),
            Mask => write!(f, "PPUMASK"),
            Status => write!(f, "PPUSTATUS"),
            OamAddr => write!(f, "OAMADDR"),
            OamData => write!(f, "OAMDATA"),
            Scroll => write!(f, "PPUSCROLL"),
            Addr => write!(f, "PPUADDR"),
            Data => write!(f, "PPUDATA"),
        }
    }
}

impl From<Address> for PpuRegister {
    fn from(addr: Address) -> Self {
        use PpuRegister::*;
        match addr.alias(PPU_REG_ADDR_BITS).as_usize() {
            0 => Ctrl,
            1 => Mask,
            2 => Status,
            3 => OamAddr,
            4 => OamData,
            5 => Scroll,
            6 => Addr,
            7 => Data,
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
struct Registers {
    ctrl: u8,
    mask: u8,
    status: u8,
    oam_addr: u8,
    scroll: [Option<u8>; 2],
    addr: [Option<u8>; 2],

    // Contains the most recently written or read value from any register. This
    // is used to mimic the behavior of the data bus between the NES's CPU and
    // PPU, which retains the value of the most recent read or write. Attempts
    // to read from a write-only register will return this retained value.
    most_recent_value: u8,
}

/// Trait representing the PPU's address bus, which is used to access the PPU's
/// address space (separate from the CPU addres space). PPU memory accesses can
/// be arbitrarily remapped by the cartridge, which is why a reference to the
/// PPU's VRAM is passed into these methods (so that the mapper can choose to
/// map a read or write to VRAM).
pub trait PpuBus {
    fn ppu_load(&mut self, vram: &Vram, palette: &[u8; 32], addr: Address) -> u8;

    fn ppu_store(&mut self, vram: &mut Vram, palette: &mut [u8; 32], addr: Address, value: u8);
}

pub struct Ppu<M> {
    registers: Registers,
    vram: Vram,
    oam: [u8; 256],
    palette: [u8; 32],
    mapper: M,
}

impl<M: PpuBus> Ppu<M> {
    pub fn with_mapper(mapper: M) -> Self {
        Self {
            registers: Registers::default(),
            vram: Vram::new(),
            oam: [0; 256],
            palette: [0; 32],
            mapper,
        }
    }

    /// Load a value from PPU memory via the mapper.
    fn mapper_load(&mut self, addr: Address) -> u8 {
        self.mapper.ppu_load(&self.vram, &self.palette, addr)
    }

    /// Store a value to PPU memory via the mapper.
    fn mapper_store(&mut self, addr: Address, value: u8) {
        self.mapper
            .ppu_store(&mut self.vram, &mut self.palette, addr, value);
    }

    /// Replace the entire contents of OAM with the given data.
    pub fn oam_dma(&mut self, oam_data: [u8; 256]) {
        self.oam = oam_data;
    }

    pub fn tick(&mut self, frame: &mut [u8]) {
        self.render_name_table(frame, NAMETABLES[0]);
    }

    /// Render the specified nametable.
    pub fn render_name_table(&mut self, frame: &mut [u8], table: Address) {
        for pos in 0..960 {
            let tile_num = self.mapper_load(table + pos as u16);
            let tile = self.load_tile(Address(0), tile_num);

            let attr_table = table + ATTRIBUTE_TABLE_OFFSET;
            let attr = self.get_attribute(attr_table, tile_num);
            let palette = self.load_palette(attr, false);

            tile.draw(frame, pos, palette);
        }
    }

    /// Get the palette index for a tile from the given attribute table.
    pub fn get_attribute(&mut self, table: Address, tile_num: u8) -> u8 {
        // Get position of the tile within the nametable's 32x30 tile grid.
        let (tile_x, tile_y) = tile_coords(tile_num);

        // Determine which byte of the attribute table contains the palette for
        // this tile's block. Since each attribute is 2-bits wide, and each
        // attribute controls the palette for a 16x16 pixel (2x2 tile) block,
        // we need to scale down the coordinates by 4.
        let attr_x = tile_x as u16 / 4;
        let attr_y = tile_y as u16 / 4;
        let attr_num = attr_y * 8 + attr_x;

        let attr_byte = self.mapper_load(table + attr_num);

        // Identify which quadrant (16x16 block) this tile falls into within the
        // byte, and obtain the attribute by shifting the value accordingly.
        let quad_x = (tile_x / 2) % 2;
        let quad_y = (tile_y / 2) % 2;

        let shift = match (quad_x, quad_y) {
            (0, 0) => 0,
            (0, 1) => 2,
            (1, 0) => 4,
            (1, 1) => 6,
            _ => unreachable!(),
        };

        (attr_byte >> shift) & 3
    }

    /// Read the pattern tables from the PPU's address space and render them as
    /// a pair of 128x128 greyscale grids. The output buffer must be at least
    /// 16 KiB in size in order to store 2 * 128 * 128 * 4 bytes (each pixel is
    /// stored as a 4-byte RGBA sequence).
    pub fn render_pattern_table(&mut self, frame: &mut [u8]) {
        assert!(frame.len() >= 0x4000);
        for table in 0..2 {
            // Get address of the nametable we're using.
            let table_addr = Address(table as u16 * 0x1000u16);
            for tile_num in 0..256 {
                // Get tile position in grid.
                let tile_x = tile_num % 16;
                let tile_y = tile_num / 16;

                // Get desired coords of upper left pixel of tile.
                let x = tile_x * 8 + (128 * table);
                let y = tile_y * 8;

                // Load and draw tile.
                let tile = self.load_tile(table_addr, tile_num as u8);
                tile.draw_at(frame, FRAME_WIDTH, x, y, GREYSCALE_PALETTE);
            }
        }
    }

    /// Load a tile from the pattern table at the specified address.
    ///
    /// Each pattern table consists of 256 8x8 tiles, with 2 bits per pixel.
    /// These two bits are not stored adjacently; instead, the low bits of the
    /// tile are stored first, followed by the high bits. As such, this method
    /// returns 2 arrays containing the low bits and high bits respectively.
    fn load_tile(&mut self, table: Address, tile_num: u8) -> Tile {
        let mut low = [0u8; 8];
        let mut high = [0u8; 8];
        let base = table + tile_num as u16 * 16;
        for i in 0..8 {
            low[i] = self.mapper_load(base + i as u16);
            high[i] = self.mapper_load(base + i as u16 + 8u16);
        }
        Tile { low, high }
    }

    /// Load a background or sprite palette from the PPU's memory.
    fn load_palette(&mut self, palette_num: u8, sprite: bool) -> Palette {
        // The palette number is a 2-bit value.
        assert!(palette_num < 5);

        let palettes = if sprite { SPRITE_PALETTES } else { BG_PALETTES };

        let addr = palettes[palette_num as usize];
        let color1 = self.mapper_load(addr);
        let color2 = self.mapper_load(addr + 1u16);
        let color3 = self.mapper_load(addr + 2u16);

        let background = self.mapper_load(BG_COLOR);

        Palette {
            background,
            color1,
            color2,
            color3,
        }
    }
}

/// The CPU can interact with the PPU via its registers, which are mapped into
/// the CPU's address space. Only the last 3 bits of the address are decoded,
/// meaning that the registers are mirrored every 8-bits.
impl<M: PpuBus> Bus for Ppu<M> {
    fn load(&mut self, addr: Address) -> u8 {
        use PpuRegister::*;

        let value = match addr.into() {
            Status => {
                // Reading the status register clears the address and scroll
                // registers.
                self.registers.scroll = [None, None];
                self.registers.addr = [None, None];

                // Lower 5 bits of status register are unused, so reading them
                // will return the residual contents of the last read/write.
                let value = self.registers.status | (0xE0 & self.registers.most_recent_value);

                // Reading the status register also clears bit 7.
                self.registers.status &= 0x7F;

                value
            }
            OamData => self.oam[self.registers.oam_addr as usize],
            Data => {
                let addr = read_ppuaddr(&self.registers.addr);
                if addr < PALETTE_BASE_ADDR {
                    // Read from PPU address space via mapper.
                    self.mapper_load(addr)
                } else {
                    let i = addr.alias(PALETTE_ADDR_BITS).as_usize();
                    self.palette[i]
                }
            }
            // All other registers are write-only, and therefore attempts to
            // read their values will just return whatever value is presently
            // on the data bus (i.e., whatever value was most recently read or
            // written).
            _ => self.registers.most_recent_value,
        };

        log::debug!(
            "Read from PPU register {}: {:#X}",
            PpuRegister::from(addr),
            value
        );

        self.registers.most_recent_value = value;

        value
    }

    fn store(&mut self, addr: Address, value: u8) {
        use PpuRegister::*;

        log::debug!(
            "Write to PPU register {}: {:#X}",
            PpuRegister::from(addr),
            value
        );

        self.registers.most_recent_value = value;
        match addr.into() {
            Ctrl => self.registers.ctrl = value,
            Mask => self.registers.mask = value,
            Status => {
                // Status register is read-only.
                log::error!("Attempted write to PPUSTATUS register: {:#X}", value);
            }
            OamAddr => self.registers.oam_addr = value,
            OamData => self.oam[self.registers.oam_addr as usize] = value,
            Scroll => double_write(&mut self.registers.scroll, value),
            Addr => double_write(&mut self.registers.addr, value),
            Data => {
                let addr = read_ppuaddr(&self.registers.addr);
                if addr < PALETTE_BASE_ADDR {
                    self.mapper_store(addr, value);
                } else {
                    let i = addr.alias(PALETTE_ADDR_BITS).as_usize();
                    self.palette[i] = value;
                }
            }
        };
    }
}

/// The PPU has its own dedicated VRAM separate from the CPU, primarily used to
/// store the nametables. Note that although the NES logically has 4 nametables,
/// the VRAM is only large enough to store 2 of them. Games can work around this
/// by mapping the remainder of the VRAM address range to the cartridge itself
/// (which presumably has additional RAM chips). Otherwise, the contents of VRAM
/// are mirrored to fill up the available address range for nametables.
pub struct Vram(pub [u8; VRAM_SIZE]);

impl Vram {
    fn new() -> Self {
        Vram([0; VRAM_SIZE])
    }
}

// Emulate the behavior of the PPUADDR and PPUSCROLL registers, which require
// the CPU to write 2 bytes. Since each register is only mapped to a single
// byte of the CPU's address space, the CPU must perform 2 writes in succession.
fn double_write(reg: &mut [Option<u8>; 2], value: u8) {
    *reg = match *reg {
        [None, None] => [Some(value), None],
        [Some(first), None] => [Some(first), Some(value)],
        _ => return,
    };
}

/// Intepret the contents of the PPUADDR register.
fn read_ppuaddr(addr: &[Option<u8>; 2]) -> Address {
    let high = addr[0].unwrap_or(0);
    let low = addr[1].unwrap_or(0);
    Address::from([low, high])
}

/// An 8x8 tile from a pattern table.
///
/// The tile is represented by two arrays containing the low and high bits of
/// each pixel respectively. Each byte in these arrays represents a row of 8
/// pixels, so to getting a pixel's value requires reading the desired bit from
/// both arrays and combining them into a 2-bit value.
#[derive(Debug, Copy, Clone)]
struct Tile {
    low: [u8; 8],
    high: [u8; 8],
}

impl Tile {
    /// Get the 2-bit value of the pixel at the specified position in the tile.
    fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        // Get bits for pixel and convert to RGBA. Note that the highest-order
        // bit is considered the "first" bit, so the bit indexes are inverted.
        let low = self.low[y] & 1 << (7 - x) > 0;
        let high = self.high[y] & 1 << (7 - x) > 0;
        Pixel::from_bits(low, high)
    }

    /// Draw this tile to a framebuffer at the specified pixel coordinates.
    ///
    /// This method makes no assumptions about frame size or tile alignment,
    /// making it suitable for implementing debug functionality that might need
    /// to draw tiles at nonstandard positions.
    fn draw_at(
        &self,
        frame: &mut [u8],
        frame_width_px: usize,
        pos_x: usize,
        pos_y: usize,
        palette: Palette,
    ) {
        for x in 0..8 {
            for y in 0..8 {
                let rgba = self.get_pixel(x, y).to_rgba(palette);
                let pos = (pos_y + y) * frame_width_px + pos_x + x;
                let offset = pos * 4; // 4 bytes per RGBA pixel.
                frame[offset..offset + 4].copy_from_slice(&rgba[..]);
            }
        }
    }

    /// Draw the tile to the framebuffer at the specified tile offset.
    ///
    /// Assumes that the screen is a 32 x 30 tile grid and the position is
    /// specified as the tile's index in that grid (from 0 to 960).
    fn draw(&self, frame: &mut [u8], pos: usize, palette: Palette) {
        let pos_x = pos % (FRAME_WIDTH / 8) * 8;
        let pos_y = pos / (FRAME_WIDTH / 8) * 8;
        self.draw_at(frame, FRAME_WIDTH, pos_x, pos_y, palette);
    }
}

/// A 2-bit pixel value from a Tile.
#[derive(Debug, Copy, Clone)]
struct Pixel(u8);

impl Pixel {
    /// Create a color from a low bit and a high bit.
    fn from_bits(low: bool, high: bool) -> Self {
        Self((high as u8) << 1 | low as u8)
    }

    /// Get this pixel's 6-bit color index using the given palette.
    fn color(&self, palette: Palette) -> u8 {
        match self.0 {
            0 => palette.background,
            1 => palette.color1,
            2 => palette.color2,
            3 => palette.color3,
            _ => unreachable!(),
        }
    }

    /// Get this pixel's RGBA value using the given palette.
    fn to_rgba(&self, palette: Palette) -> [u8; 4] {
        let color = self.color(palette) as usize;
        let mut rgba = [0xFFu8; 4];
        rgba[..3].copy_from_slice(&NES_COLORS[color * 3..color * 3 + 3]);
        rgba
    }
}

/// A palette value, consisting of a background color (which is shared by all
/// palettes) and 3 other colors. The color values are used as indexes for
/// looking up the color's RGB value from a NES palette file.
#[derive(Debug, Copy, Clone)]
struct Palette {
    background: u8,
    color1: u8,
    color2: u8,
    color3: u8,
}

/// Get the coordinates for the specified tile within a nametable.
fn tile_coords(tile_num: u8) -> (u8, u8) {
    (tile_num % 32, tile_num / 32)
}
