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

#[derive(Copy, Clone)]
pub enum IoRegs {
    TIMA = 0xff05,
    TMA = 0xff06,
    TAC = 0xff07,
    IF = 0xff0f,
    IE = 0xffff,
}
