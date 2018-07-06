use super::{get_u16, locations, set_u16, sizes};
use bit_ops::BitGetSet;
use std::default::Default;
use std::ops::{Index, IndexMut};

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
    oam: [u8; sizes::OAM],
    pub regs: VideoRegisters,
}

impl VideoMemory {
    pub(super) fn new() -> VideoMemory {
        let vram = [0; sizes::VRAM];
        let oam = [0; sizes::OAM];
        VideoMemory {
            vram,
            oam,
            regs: Default::default(),
        }
    }

    pub fn get_u16(&self, index: usize) -> u16 {
        match index {
            locations::VRAM_START...locations::VRAM_END => {
                get_u16(&self.vram, index - locations::VRAM_START)
            }
            locations::OAM_START...locations::OAM_END => {
                get_u16(&self.oam, index - locations::OAM_START)
            }
            _ => panic!("Invalid index for VideoMemory"),
        }
    }

    pub(super) fn set_u16(&mut self, index: usize, value: u16) {
        match index {
            locations::VRAM_START...locations::VRAM_END => {
                set_u16(&mut self.vram, index - locations::VRAM_START, value);
            }
            locations::OAM_START...locations::OAM_END => {
                set_u16(&mut self.oam, index - locations::OAM_START, value);
            }
            _ => panic!("Invalid index for VideoMemory"),
        }
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
        match index {
            locations::VRAM_START...locations::VRAM_END => {
                &self.vram[index - locations::VRAM_START]
            }
            locations::OAM_START...locations::OAM_END => &self.oam[index - locations::OAM_START],
            _ => panic!("Invalid index for VideoMemory"),
        }
    }
}

impl IndexMut<usize> for VideoMemory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        match index {
            locations::VRAM_START...locations::VRAM_END => {
                &mut self.vram[index - locations::VRAM_START]
            }
            locations::OAM_START...locations::OAM_END => {
                &mut self.oam[index - locations::OAM_START]
            }
            _ => panic!("Invalid index for VideoMemory"),
        }
    }
}
