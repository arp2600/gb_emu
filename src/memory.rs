use std::fs;

const ROM_BANK_SIZE: usize = 0x4000;
const HRAM_SIZE: usize = 0x7e;

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

    fn get_u8(&self, index: usize) -> u8 {
        match index {
            0x0...0x3fff => self.zero_bank[index],
            _ => panic!("Bad read at {}", index),
        }
    }
}

pub struct Memory<'a> {
    boot_rom: &'a mut [u8],
    cartridge: &'a mut Cartridge,
    hram: [u8; HRAM_SIZE],
}

impl<'a> Memory<'a> {
    pub fn new(boot_rom: &'a mut [u8], cartridge: &'a mut Cartridge) -> Memory<'a> {
        Memory {
            boot_rom,
            cartridge,
            hram: [0; HRAM_SIZE],
        }
    }

    pub fn set_u8(&mut self, index: u16, value: u8) {
        if (index as usize) < self.boot_rom.len() {
            self.boot_rom[index as usize] = value;
        }
        // match index {
        //     0x0000...0x3FFF => println!("write ROM bank 0"),
        //     0x4000...0x7FFF => println!("write ROM bank x"),
        //     0x8000...0x9FFF => println!("VRAM[{}] = {}", index - 0x8000, value),
        //     0xA000...0xBFFF => println!("RAM[{}] = {}", index - 0xA000, value),
        //     0xC000...0xCFFF => println!("WRAM0[{}] = {}", index - 0xC000, value),
        //     0xD000...0xDFFF => println!("WRAM1[{}] = {}", index - 0xD000, value),
        //     0xE000...0xFDFF => println!("ECHO[{}] = {}", index - 0xE000, value),
        //     0xFE00...0xFE9F => println!("OAM[{}] = {}", index - 0xFE00, value),
        //     0xFEA0...0xFEFF => println!("Error"),
        //     0xFF00...0xFF7F => println!("IO[{}] = {}", index - 0xFF00, value),
        //     0xFF80...0xFFFE => println!("HRAM[{}] = {}", index - 0xFF00, value),
        //     0xFFFF => println!("enable interrups = {}", value),
        //     _ => panic!("error writing!"),
        // }

        // 0x0000-0x3FFF   16KB ROM Bank 00     (in cartridge, fixed at bank 00)
        // 0x4000-0x7FFF   16KB ROM Bank 01..NN (in cartridge, switchable bank number)
        // 0x8000-0x9FFF   8KB Video RAM (VRAM) (switchable bank 0-1 in CGB Mode)
        // 0xA000-0xBFFF   8KB External RAM     (in cartridge, switchable bank, if any)
        // 0xC000-0xCFFF   4KB Work RAM Bank 0 (WRAM)
        // 0xD000-0xDFFF   4KB Work RAM Bank 1 (WRAM)  (switchable bank 1-7 in CGB Mode)
        // 0xE000-0xFDFF   Same as C000-DDFF (ECHO)    (typically not used)
        // 0xFE00-0xFE9F   Sprite Attribute Table (OAM)
        // 0xFEA0-0xFEFF   Not Usable
        // 0xFF00-0xFF7F   I/O Ports
        // 0xFF80-0xFFFE   High RAM (HRAM)
        // 0xFFFF        Interrupt Enable Register
    }

    pub fn get_u8(&self, index: u16) -> u8 {
        let index = index as usize;
        match index {
            x if x < self.boot_rom.len() => self.boot_rom[x],
            0x0...0x8000 => self.cartridge.get_u8(index),
            0xff80...0xfffe => self.hram[index - 0xff80],
            x => panic!("Bad memory read at {}", x),
        }
    }

    pub fn get_u16(&self, index: u16) -> u16 {
        let index = index as usize;
        match index {
            x if x < self.boot_rom.len() => get_u16(&self.boot_rom, index),
            0xff80...0xfffe => get_u16(&self.hram, index - 0xff80),
            x => {
                let location = index_to_location(x);
                panic!("Bad read: {}", location);
            }
        }
    }

    pub fn set_u16(&mut self, index: u16, value: u16) {
        let index = index as usize;
        match index {
            0xff80...0xfffe => set_u16(&mut self.hram, index - 0xff80, value),
            x => {
                let location = index_to_location(x);
                panic!("Bad write: {}", location);
            }
        }
    }
}

pub fn index_to_location(index: usize) -> String {
    match index {
        0x0000...0x3FFF => format!("ROM bank 0[{}]", index),
        0x4000...0x7FFF => format!("ROM bank n[{}]", index),
        0x8000...0x9FFF => format!("VRAM[{}]", index),
        0xA000...0xBFFF => format!("RAM[{}]", index),
        0xC000...0xCFFF => format!("WRAM0[{}]", index),
        0xD000...0xDFFF => format!("WRAM1[{}]", index),
        0xE000...0xFDFF => format!("ECHO[{}]", index),
        0xFE00...0xFE9F => format!("OAM[{}]", index),
        0xFEA0...0xFEFF => format!("Not usable[{}]", index),
        0xFF00...0xFF7F => format!("IO[{}]", index),
        0xFF80...0xFFFE => format!("HRAM[{}]", index),
        0xFFFF => format!("InterruptEnableRegister"),
        _ => panic!("Bad index {}", index),
    }
}

fn get_u16(mem: &[u8], index: usize) -> u16 {
    let high = mem[index + 1] as u16;
    let low = mem[index] as u16;
    (high << 8) | low
}

fn set_u16(mem: &mut [u8], index: usize, value: u16) {
    let high = value >> 8;
    let low = value & 0xff;
    mem[index + 1] = high as u8;
    mem[index] = low as u8;
}
