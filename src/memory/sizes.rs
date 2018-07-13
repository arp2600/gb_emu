use super::*;

const KILOBYTE: usize = 1024;

pub const VRAM: usize = 8 * KILOBYTE;
pub const OAM: usize = OAM_END - OAM_START + 1;
pub const WRAM: usize = 8 * KILOBYTE;
pub const IO: usize = IO_END - IO_START + 1;
pub const HRAM: usize = 127;
pub const EXRAM: usize = EXRAM_END - EXRAM_START + 1;
