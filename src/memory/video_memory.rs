use std::ops::{Index, IndexMut};
use super::{get_u16, set_u16, sizes, locations};

pub struct VideoMemory {
    vram: [u8; sizes::VRAM],
}

impl VideoMemory {
    pub(super) fn new() -> VideoMemory {
        let vram = [0; sizes::VRAM];
        VideoMemory { vram }
    }

    pub(super) fn get_u16(&self, index: usize) -> u16 {
        get_u16(&self.vram, index - locations::VRAM_START)
    }

    pub(super) fn set_u16(&mut self, index: usize, value: u16) {
        set_u16(&mut self.vram, index - locations::VRAM_START, value);
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
