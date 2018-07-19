mod mbc1;
mod mbc2;
mod rom_only;
use self::mbc1::Mbc1;
use self::mbc2::Mbc2;
use self::rom_only::RomOnly;
use memory::locations::*;
use std::fs;

const ROM_BANK_SIZE: usize = 0x4000;

enum CartType {
    RomOnly,
    Mbc1,
    Mbc2,
}

impl CartType {
    fn try_from_u8(value: u8) -> Result<CartType, String> {
        match value {
            0x00 => Ok(CartType::RomOnly),
            0x01 | 0x02 | 0x03 => Ok(CartType::Mbc1),
            0x05 | 0x06 => Ok(CartType::Mbc2),
            _ => Err(format!("Unknown cart type {:#04x}", value)),
        }
    }
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
        let cart_type = CartType::try_from_u8(full_rom[CARTRIDGE_TYPE]).unwrap();
        match cart_type {
            CartType::RomOnly => {
                let mut rom = [0; ROM_BANK_SIZE * 2];
                let data = &full_rom[..rom.len()];
                rom.copy_from_slice(data);
                Box::new(RomOnly { rom })
            }
            CartType::Mbc1 => Box::new(Mbc1::new(&full_rom)),
            CartType::Mbc2 => Box::new(Mbc2::new(&full_rom)),
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
