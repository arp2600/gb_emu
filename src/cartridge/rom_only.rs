use super::{Cartridge, ROM_BANK_SIZE};

pub struct RomOnly {
    pub rom: [u8; ROM_BANK_SIZE * 2],
}

impl Cartridge for RomOnly {
    fn get_u8(&self, index: usize) -> u8 {
        self.rom[index]
    }

    fn set_u8(&mut self, index: usize, value: u8) {
        eprintln!(
            "warning: attempting to write to RomOnly cartridge at {:#06x} with value {:#04x}",
            index, value
        );
    }
}
