pub const KILOBYTE: usize = 1024;

pub const ROM_0_START: usize = 0x0000;
pub const ROM_0_END: usize = 0x3fff;

pub const ROM_N_START: usize = 0x4000;
pub const ROM_N_END: usize = 0x7fff;

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9fff;
pub const VRAM_SIZE: usize = 8 * KILOBYTE;

pub const EXRAM_START: usize = 0xa000;
pub const EXRAM_END: usize = 0xbfff;

pub const WRAM_START: usize = 0xc000;
pub const WRAM_SIZE: usize = 8 * KILOBYTE;
pub const WRAM_END: usize = 0xdfff;

pub const WRAM_ECHO_START: usize = 0xe000;
pub const WRAM_ECHO_END: usize = 0xfdff;

pub const OAM_START: usize = 0xfe00;
pub const OAM_END: usize = 0xfe9f;
pub const OAM_SIZE: usize = OAM_END - OAM_START + 1;

pub const IO_START: usize = 0xff00;
pub const IO_END: usize = 0xff7f;
pub const IO_SIZE: usize = IO_END - IO_START + 1;

pub const HRAM_START: usize = 0xff80;
pub const HRAM_SIZE: usize = 127;
pub const HRAM_END: usize = 0xfffe;

pub const INTERRUPT_ENABLE_REG: usize = 0xffff;

pub const TILE_MAP_1: u16 = 0x9800;
pub const TILE_MAP_2: u16 = 0x9c00;
pub const TILE_DATA_1: u16 = 0x8800;
pub const TILE_DATA_2: u16 = 0x8000;
pub const SPRITE_PATTERN_TABLE: u16 = 0x8000;
pub const SPRITE_ATTRIBUTE_TABLE: u16 = 0xfe00;

pub mod io_regs {
    pub const JOYP: usize = 0xff00;
    pub const TIMA: usize = 0xff05;
    pub const TMA: usize = 0xff06;
    pub const TAC: usize = 0xff07;
    pub const IF: usize = 0xff0f;
    pub const IE: usize = 0xffff;
    pub const LCDC: usize = 0xff40;
    pub const STAT: usize = 0xff41;
    pub const SCY: usize = 0xff42;
    // SCX = 0xff43,
    pub const LY: usize = 0xff44;
    pub const LYC: usize = 0xff45;
    // WY = 0xff4a,
    // WX = 0xff4b,
    pub const DMA: usize = 0xff46;
    pub const BGP: usize = 0xff47;
    // OBP0 = 0xff48,
    // OBP1 = 0xff49,
    // HDMA1 = 0xff51,
    // HDMA2 = 0xff52,
    // HDMA3 = 0xff53,
    // HDMA4 = 0xff54,
    // HDMA5 = 0xff55,
    // BCPS = 0xff68,
    // BCPD = 0xff69,
    // OCPS = 0xff6a,
    // OCPD = 0xff6b,
}
