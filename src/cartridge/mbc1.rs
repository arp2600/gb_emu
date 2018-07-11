use memory::locations::*;
use super::{Cartridge, ROM_BANK_SIZE};

pub struct Mbc1 {
    rom_bank_zero: Vec<u8>,
    other_rom_banks: Vec<Vec<u8>>,
    rom_bank_index: usize,
}

impl Mbc1 {
    pub fn new(data: &[u8]) -> Mbc1 {
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
            ROM_N_START...ROM_N_END => {
                let bank = &self.other_rom_banks[self.rom_bank_index - 1];
                bank[index - ROM_BANK_SIZE]
            }
            _ => unimplemented!(),
        }
    }

    fn set_u8(&mut self, index: usize, value: u8) {
        match index {
            EXRAM_START...EXRAM_END => unimplemented!(),
            0x0000...0x1fff => unimplemented!("ram enable"),
            0x2000...0x3fff => {
                match value {
                    0 => self.rom_bank_index = 1,
                    _ => self.rom_bank_index = usize::from(value),
                }
            }
            0x4000...0x5fff => unimplemented!("ram bank number"),
            0x6000...0x7fff => unimplemented!("rom/ram mode select"),
            _ => panic!("bad write index"),
        }
    }
}
