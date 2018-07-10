use std::fs;
use memory::locations::*;

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

        if full_rom.len() / 1024 > 32 {
            panic!(
                "Roms size {} KB is not supported yet. Max size is 32 KB",
                full_rom.len() / 1024
            );
        }
        assert_eq!(full_rom.len(), ROM_BANK_SIZE * 2);

        if cart_type == CartType::RomOnly as u8 {
            let mut rom = [0; ROM_BANK_SIZE * 2];
            let data = &full_rom[..rom.len()];
            rom.copy_from_slice(data); 
            Box::new(RomOnly {
                rom,
            })
        } else if cart_type  == CartType::Mbc1 as u8 {
            Box::new(Mbc1::new(&full_rom))
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

    fn set_u8(&mut self, index: usize, value: u8) {
        eprintln!("warning: attempting to write to RomOnly cartridge at {:#06x} with value {:#04x}", index, value);
    }
}

struct Mbc1 {
    rom_bank_zero: Vec<u8>,
    other_rom_banks: Vec<Vec<u8>>,
    rom_bank_index: usize,
}

impl Mbc1 {
    fn new(data: &[u8]) -> Mbc1 {
        assert!(data.len() > ROM_BANK_SIZE);

        let (lower, upper) = data.split_at(ROM_BANK_SIZE);
        let rom_bank_zero = lower.to_vec();

        let mut other_rom_banks = Vec::new();
        for bank in upper.chunks(ROM_BANK_SIZE) {
            other_rom_banks.push(bank.to_vec());
        }

        Mbc1 { rom_bank_zero, other_rom_banks, rom_bank_index: 1 }
    }
}

impl Cartridge for Mbc1 {
    fn get_u8(&self, index: usize) -> u8 {
        match index {
            ROM_0_START...ROM_0_END => self.rom_bank_zero[index],
            ROM_N_START...ROM_N_END => self.other_rom_banks[self.rom_bank_index - 1][index - ROM_BANK_SIZE],
            _ => unimplemented!(),
        }
    }

    fn set_u8(&mut self, index: usize, value: u8) {
        match index {
            EXRAM_START...EXRAM_END => unimplemented!(),
            0x0000...0x1fff => unimplemented!("ram enable"),
            0x2000...0x3fff => unimplemented!("rom bank number"),
            0x4000...0x5fff => unimplemented!("ram bank number"),
            0x6000...0x7fff => unimplemented!("rom/ram mode select"),
            _ => panic!("bad write index"),
        }
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
