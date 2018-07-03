use bit_ops::BitGetSet;
use memory::{Memory, io_regs, locations};

pub struct DrawData<'a> {
    pub ly: u8,
    pub bgp: u8,
    pub scy: u8,
    lcdc: u8,
    obp0: u8,
    obp1: u8,
    pub memory: &'a Memory,
}

impl<'a> DrawData<'a> {
    pub fn new(regs: &'a mut LCDRegisters) -> DrawData<'a> {
        let ly = regs.get_ly();
        let bgp = regs.get_bgp();
        let scy = regs.get_scy();
        let lcdc = regs.get_lcdc();
        let obp0 = regs.memory.get_u8(io_regs::OBP0 as u16);
        let obp1 = regs.memory.get_u8(io_regs::OBP1 as u16);
        let memory = &regs.memory;

        DrawData { ly, bgp, scy, lcdc, obp0, obp1, memory }
    }

    pub fn get_bg_tilemap_display_select(&self) -> u16 {
        if self.lcdc.get_bit(3) {
            locations::TILE_MAP_2
        } else {
            locations::TILE_MAP_1
        }
    }

    pub fn get_tile_data_select(&self) -> u16 {
        if self.lcdc.get_bit(4) {
            locations::TILE_DATA_2
        } else {
            locations::TILE_DATA_1
        }
    }

    pub fn are_sprites_enabled(&self) -> bool {
        self.lcdc.get_bit(1)
    }

    pub fn get_obp(&self, num: u8) -> u8 {
        if num == 1 {
            self.obp1
        } else {
            self.obp0
        }
    }
}

pub struct LCDRegisters<'a> {
    pub memory: &'a mut Memory,
    lcdc: Option<u8>,
    ly: Option<u8>,
    lyc: Option<u8>,
    stat: Option<u8>,
    scy: Option<u8>,
    bgp: Option<u8>,
}

macro_rules! create_getter {
    ( $name:ident, $member:ident, $location:expr ) => {
        pub fn $name(&mut self) -> u8 {
            match self.$member {
                Some(x) => x,
                None => {
                    let x = self.memory.get_io($location);
                    self.$member = Some(x);
                    x
                }
            }
        }
    };
}

macro_rules! create_setter {
    ( $name:ident, $member:ident, $location:expr ) => {
        pub fn $name(&mut self, value: u8) {
            self.$member = Some(value);
            self.memory.set_io($location, value);
        }
    };
}

impl<'a> LCDRegisters<'a> {
    pub fn new(memory: &mut Memory) -> LCDRegisters {
        LCDRegisters {
            memory,
            lcdc: None,
            ly: None,
            lyc: None,
            stat: None,
            scy: None,
            bgp: None,
        }
    }

    create_getter!(get_lcdc, lcdc, io_regs::LCDC);

    create_getter!(get_ly, ly, io_regs::LY);
    create_setter!(set_ly, ly, io_regs::LY);

    create_getter!(get_lyc, lyc, io_regs::LYC);

    create_getter!(get_scy, scy, io_regs::SCY);

    create_getter!(get_bgp, bgp, io_regs::BGP);

    create_getter!(get_stat, stat, io_regs::STAT);
    pub fn set_stat(&mut self, value: u8) {
        self.stat = Some(value);
        self.memory.set_stat(value);
    }

    pub fn check_enabled(&mut self) -> bool {
        self.get_lcdc().get_bit(7)
    }

    pub fn set_interrupt_bit(&mut self) {
        let if_reg = self.memory.get_io(io_regs::IF).set_bit(0);
        self.memory.set_io(io_regs::IF, if_reg);
    }

    pub fn set_lcd_mode(&mut self, mode: u8) {
        let stat = self.get_stat();
        self.set_stat(stat & 0b1111_1100 | mode & 0b11);
    }

    pub fn set_coincidence_flag(&mut self, state: bool) {
        let stat = self.get_stat();
        if state {
            self.set_stat(stat.set_bit(2));
        } else {
            self.set_stat(stat.reset_bit(2));
        }
    }
}
