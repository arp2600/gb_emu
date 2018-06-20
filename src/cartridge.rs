use std::fs;

const ROM_BANK_SIZE: usize = 0x4000;

pub struct Cartridge {
    zero_bank: Vec<u8>,
    other_banks: Vec<Vec<u8>>,
}

impl Cartridge {
    pub fn from_file(file_path: &str) -> Cartridge {
        let mut full_rom = fs::read(file_path).unwrap();
        if full_rom.len() / 1024 > 32 {
            panic!(
                "Roms size {} KB is not supported yet. Max size is 32 KB",
                full_rom.len() / 1024
            );
        }
        let mut remaining = full_rom.split_off(ROM_BANK_SIZE);
        let mut other_banks = Vec::new();
        while remaining.len() >= ROM_BANK_SIZE {
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
            other_banks,
        }
    }

    pub fn get_u8(&self, index: usize) -> u8 {
        match index {
            0x0...0x3fff => self.zero_bank[index],
            0x4000...0x7fff => self.other_banks[0][index - 0x4000],
            _ => panic!("Bad read at {}", index),
        }
    }

    pub fn get_u16(&self, index: usize) -> u16 {
        let low = u16::from(self.get_u8(index));
        let high = u16::from(self.get_u8(index + 1));
        (high << 8) | low
    }
}

#[cfg(test)]
mod tests {
    use cartridge::Cartridge;
    use cartridge::ROM_BANK_SIZE;

    impl Cartridge {
        pub fn create_dummy() -> Cartridge {
            let zero_bank = vec![0; ROM_BANK_SIZE];
            let other_banks = {
                let x = vec![0; ROM_BANK_SIZE];
                vec![x]
            };
            Cartridge {
                zero_bank,
                other_banks,
            }
        }
    }
}
