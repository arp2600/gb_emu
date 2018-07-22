use super::{Cartridge, ROM_BANK_SIZE};
use memory::locations::*;
use memory::sizes;

pub struct Mbc3 {
    rom_bank_zero: Vec<u8>,
    other_rom_banks: Vec<Vec<u8>>,
    rom_bank_index: usize,
    ram_enabled: bool,
    ram_bank_index: usize,
    ram_banks: Vec<Vec<u8>>,
}

impl Mbc3 {
    pub fn new(data: &[u8]) -> Mbc3 {
        assert!(data.len() > ROM_BANK_SIZE);

        let (lower, upper) = data.split_at(ROM_BANK_SIZE);
        let rom_bank_zero = lower.to_vec();

        let mut other_rom_banks = Vec::new();
        for bank in upper.chunks(ROM_BANK_SIZE) {
            other_rom_banks.push(bank.to_vec());
        }

        let ram_banks = {
            let mut x = Vec::new();
            for _ in 0..4 {
                x.push(vec![0; sizes::EXRAM]);
            }
            x
        };

        Mbc3 {
            rom_bank_zero,
            other_rom_banks,
            rom_bank_index: 1,
            ram_enabled: false,
            ram_bank_index: 0,
            ram_banks,
        }
    }
}

impl Cartridge for Mbc3 {
    fn get_u8(&self, index: usize) -> u8 {
        match index {
            ROM_0_START...ROM_0_END => self.rom_bank_zero[index],
            ROM_N_START...ROM_N_END => {
                assert!(self.rom_bank_index - 1 < self.other_rom_banks.len());
                let bank = &self.other_rom_banks[self.rom_bank_index - 1];
                assert!(index - ROM_BANK_SIZE < bank.len());
                bank[index - ROM_BANK_SIZE]
            }
            EXRAM_START...EXRAM_END => {
                let bank = &self.ram_banks[self.ram_bank_index];
                bank[index - EXRAM_START]
            }
            _ => unreachable!(),
        }
    }

    fn set_u8(&mut self, index: usize, value: u8) {
        match index {
            EXRAM_START...EXRAM_END => {
                let bank = &mut self.ram_banks[self.ram_bank_index];
                bank[index - EXRAM_START] = value;
            }
            0x0000...0x1fff => match value & 0x0f {
                0x0a => self.ram_enabled = true,
                _ => self.ram_enabled = false,
            },
            0x2000...0x3fff => {
                let new_bank = {
                    let x = self.rom_bank_index & 0b1000_0000;
                    x | usize::from(value & 0b0111_1111)
                };
                self.rom_bank_index = match new_bank {
                    0x00 => 0x01,
                    _ => new_bank,
                };
            }
            0x4000...0x5fff => match value {
                0x00...0x03 => {
                    self.ram_bank_index = usize::from(value & 0b11);
                }
                _ => unimplemented!("rtc register select {}", value),
            },
            0x6000...0x7fff => eprintln!("latch clock data {}", value),
            _ => panic!("bad write index"),
        }
    }
}
