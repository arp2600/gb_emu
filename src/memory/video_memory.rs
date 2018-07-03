use std::ops::{Index, IndexMut};
use std::default::Default;
use super::{get_u16, set_u16, sizes, locations};
use bit_ops::BitGetSet;

#[derive(Default)]
pub struct VideoRegisters {
    pub lcdc: u8,
    pub ly: u8,
    pub lyc: u8,
    pub stat: u8,
    pub scy: u8,
    pub bgp: u8,
    pub obp0: u8,
    pub obp1: u8,
    pub vblank_interrupt_enabled: bool,
}

pub struct VideoMemory {
    vram: [u8; sizes::VRAM],
    pub regs: VideoRegisters,
}

impl VideoMemory {
    pub(super) fn new() -> VideoMemory {
        VideoMemory { vram: [0; sizes::VRAM], regs: Default::default() }
    }

    pub fn get_u16(&self, index: usize) -> u16 {
        get_u16(&self.vram, index - locations::VRAM_START)
    }

    pub(super) fn set_u16(&mut self, index: usize, value: u16) {
        set_u16(&mut self.vram, index - locations::VRAM_START, value);
    }

    pub fn get_bg_tilemap_display_select(&self) -> u16 {
        if self.regs.lcdc.get_bit(3) {
            locations::TILE_MAP_2
        } else {
            locations::TILE_MAP_1
        }
    }

    pub fn get_tile_data_select(&self) -> u16 {
        if self.regs.lcdc.get_bit(4) {
            locations::TILE_DATA_2
        } else {
            locations::TILE_DATA_1
        }
    }

    pub fn are_sprites_enabled(&self) -> bool {
        self.regs.lcdc.get_bit(1)
    }

    pub fn get_obp(&self, num: u8) -> u8 {
        if num == 1 {
            self.regs.obp1
        } else {
            self.regs.obp0
        }
    }

    pub fn check_enabled(&mut self) -> bool {
        self.regs.lcdc.get_bit(7)
    }

    pub fn set_lcd_mode(&mut self, mode: u8) {
        self.regs.stat = self.regs.stat & 0b1111_1100 | mode & 0b11;
    }

    pub fn set_coincidence_flag(&mut self, state: bool) {
        if state {
            self.regs.stat = self.regs.stat.set_bit(2);
        } else {
            self.regs.stat = self.regs.stat.reset_bit(2);
        }
    }
}

impl Index<usize> for VideoMemory {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.vram[index - locations::VRAM_START]
    }
}

impl IndexMut<usize> for VideoMemory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.vram[index - locations::VRAM_START]
    }
}
