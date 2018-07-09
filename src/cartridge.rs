use std::fs;
use memory::locations;

const ROM_BANK_SIZE: usize = 0x4000;

enum CartType {
    RomOnly = 0x0,
    Mbc1 = 0x1,
}

pub trait Cartridge {
    fn get_u8(&self, index: usize) -> u8;
}

impl Cartridge {
    pub fn from_file(file_path: &str) -> Box<Cartridge> {
        let full_rom = fs::read(file_path).unwrap();
        if full_rom.len() < 0x150 {
            panic!("ROM shorter than header length");
        }
        let cart_type = full_rom[locations::CARTRIDGE_TYPE];

        if full_rom.len() / 1024 > 32 {
            panic!(
                "Roms size {} KB is not supported yet. Max size is 32 KB",
                full_rom.len() / 1024
            );
        }
        assert_eq!(full_rom.len(), ROM_BANK_SIZE * 2);
        let mut rom = [0; ROM_BANK_SIZE * 2];
        let data = &full_rom[..rom.len()];
        rom.copy_from_slice(data); 

        if cart_type == CartType::RomOnly as u8 {
            Box::new(RomOnly {
                rom,
            })
        } else if cart_type  == CartType::Mbc1 as u8 {
            Box::new(Mbc1 {
                rom,
            })
        } else {
            panic!("Cartridge {:#04x} not implemented", cart_type);
        }
    }
}

struct RomOnly {
    rom: [u8; ROM_BANK_SIZE * 2],
}

impl Cartridge for RomOnly {
    fn get_u8(&self, index: usize) -> u8 {
        self.rom[index]
    }
}

struct Mbc1 {
    rom: [u8; ROM_BANK_SIZE * 2],
}

impl Cartridge for Mbc1 {
    fn get_u8(&self, index: usize) -> u8 {
        self.rom[index]
    }
}



#[cfg(test)]
mod tests {
    use cartridge::{Cartridge, RomOnly};
    use cartridge::ROM_BANK_SIZE;

    impl Cartridge {
        pub fn create_dummy() -> Box<Cartridge> {
            let rom = [0; ROM_BANK_SIZE * 2];
            Box::new(RomOnly {
                rom,
            })
        }
    }
}
