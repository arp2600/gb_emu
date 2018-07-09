pub const CARTRIDGE_TYPE: usize = 0x147;

pub const ROM_0_START: usize = 0x0000;
pub const ROM_0_END: usize = 0x3fff;

pub const ROM_N_START: usize = 0x4000;
pub const ROM_N_END: usize = 0x7fff;

pub const VRAM_START: usize = 0x8000;
pub const VRAM_END: usize = 0x9fff;

pub const EXRAM_START: usize = 0xa000;
pub const EXRAM_END: usize = 0xbfff;

pub const WRAM_START: usize = 0xc000;
pub const WRAM_END: usize = 0xdfff;

pub const WRAM_ECHO_START: usize = 0xe000;
pub const WRAM_ECHO_END: usize = 0xfdff;

pub const OAM_START: usize = 0xfe00;
pub const OAM_END: usize = 0xfe9f;

pub const IO_START: usize = 0xff00;
pub const IO_END: usize = 0xff7f;

pub const HRAM_START: usize = 0xff80;
pub const HRAM_END: usize = 0xfffe;

pub const INTERRUPT_ENABLE_REG: usize = 0xffff;

pub const TILE_MAP_1: u16 = 0x9800;
pub const TILE_MAP_2: u16 = 0x9c00;
pub const TILE_DATA_1: u16 = 0x8800;
pub const TILE_DATA_2: u16 = 0x8000;
pub const SPRITE_PATTERN_TABLE: u16 = 0x8000;
pub const SPRITE_ATTRIBUTE_TABLE: u16 = 0xfe00;
