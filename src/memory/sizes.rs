use super::locations;

const KILOBYTE: usize = 1024;

pub const VRAM: usize = 8 * KILOBYTE;
pub const OAM: usize = locations::OAM_END - locations::OAM_START + 1;
pub const WRAM: usize = 8 * KILOBYTE;
pub const IO: usize = locations::IO_END - locations::IO_START + 1;
pub const HRAM: usize = 127;
pub const EXRAM: usize = locations::EXRAM_END - locations::EXRAM_START + 1;
