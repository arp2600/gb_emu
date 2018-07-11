mod mbc1;
mod rom_only;
use self::mbc1::Mbc1;
use self::rom_only::RomOnly;
use memory::locations::*;
use std::fs;

const ROM_BANK_SIZE: usize = 0x4000;

enum CartType {
    RomOnly = 0x0,
    Mbc1 = 0x1,
}

pub trait Cartridge {
    fn get_u8(&self, index: usize) -> u8;
    fn set_u8(&mut self, index: usize, value: u8);
}

impl Cartridge {
    pub fn from_file(file_path: &str) -> Box<Cartridge> {
        let full_rom = fs::read(file_path).unwrap();
        if full_rom.len() < 0x150 {
            panic!("ROM shorter than header length");
        }
        let cart_type = full_rom[CARTRIDGE_TYPE];

        if cart_type == CartType::RomOnly as u8 {
            let mut rom = [0; ROM_BANK_SIZE * 2];
            let data = &full_rom[..rom.len()];
            rom.copy_from_slice(data);
            Box::new(RomOnly { rom })
        } else if cart_type == CartType::Mbc1 as u8 {
            Box::new(Mbc1::new(&full_rom))
        } else {
            panic!("Cartridge {:#04x} not implemented", cart_type);
        }
    }
}

#[cfg(test)]
mod tests {
    use cartridge::ROM_BANK_SIZE;
    use cartridge::{Cartridge, RomOnly};

    impl Cartridge {
        pub fn create_dummy() -> Box<Cartridge> {
            let rom = [0; ROM_BANK_SIZE * 2];
            Box::new(RomOnly { rom })
        }
    }
}
