use std::fs;

const ROM_BANK_SIZE: usize = 0x4000;

pub struct Cartridge {
    zero_bank: Vec<u8>,
    other_banks: Vec<Vec<u8>>,
}

impl Cartridge {
    pub fn from_file(file_path: &str) -> Cartridge {
        let mut full_rom = fs::read(file_path).unwrap();
        let mut remaining = full_rom.split_off(ROM_BANK_SIZE);
        let mut other_banks = Vec::new();
        while remaining.len() > ROM_BANK_SIZE {
            let tail = remaining.split_off(ROM_BANK_SIZE);
            other_banks.push(remaining);
            remaining = tail;
        }

        assert_eq!(full_rom.len(), ROM_BANK_SIZE);
        for bank in &other_banks {
            assert_eq!(bank.len(), ROM_BANK_SIZE);
        }

        Cartridge {
            zero_bank: full_rom,
            other_banks: other_banks,
        }
    }

    pub fn get_u8(&self, index: usize) -> u8 {
        match index {
            0x0...0x3fff => self.zero_bank[index],
            _ => panic!("Bad read at {}", index),
        }
    }
}