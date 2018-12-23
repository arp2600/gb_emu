use super::{Cartridge, ROM_BANK_SIZE};
use crate::memory::locations::*;

const RAMEND: usize = EXRAM_START + 512;

pub struct Mbc2 {
    rom_bank_zero: Vec<u8>,
    other_rom_banks: Vec<Vec<u8>>,
    rom_bank_index: usize,
    ram: Vec<u8>,
    ram_enabled: bool,
}

impl Mbc2 {
    pub fn new(data: &[u8]) -> Mbc2 {
        assert!(data.len() > ROM_BANK_SIZE);

        let (lower, upper) = data.split_at(ROM_BANK_SIZE);
        let rom_bank_zero = lower.to_vec();

        let mut other_rom_banks = Vec::new();
        for bank in upper.chunks(ROM_BANK_SIZE) {
            other_rom_banks.push(bank.to_vec());
        }

        let ram = vec![0; 512];

        Mbc2 {
            rom_bank_zero,
            other_rom_banks,
            rom_bank_index: 1,
            ram,
            ram_enabled: false,
        }
    }
}

impl Cartridge for Mbc2 {
    fn get_u8(&self, index: usize) -> u8 {
        match index {
            ROM_0_START...ROM_0_END => self.rom_bank_zero[index],
            ROM_N_START...ROM_N_END => {
                assert!(self.rom_bank_index - 1 < self.other_rom_banks.len());
                let bank = &self.other_rom_banks[self.rom_bank_index - 1];
                assert!(index - ROM_BANK_SIZE < bank.len());
                bank[index - ROM_BANK_SIZE]
            }
            EXRAM_START...RAMEND => self.ram[index - EXRAM_START] & 0xf,
            _ => unreachable!(),
        }
    }

    fn set_u8(&mut self, index: usize, value: u8) {
        match index {
            EXRAM_START...RAMEND => {
                self.ram[index - EXRAM_START] = value & 0xf;
            }
            0x0000...0x1fff => {
                self.ram_enabled = index & 0x100 == 0;
            }
            0x2000...0x3fff => {
                if index & 0x100 != 1 {
                    self.rom_bank_index = usize::from(value & 0xf);
                }
            }
            _ => panic!("bad write index"),
        }
    }

    fn get_ram(&self) -> Vec<u8> {
        unimplemented!();
    }

    fn set_ram(&mut self, _: &[u8]) {
        unimplemented!();
    }
}
