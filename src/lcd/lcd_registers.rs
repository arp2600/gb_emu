use bit_ops::BitGetSet;
use memory::Memory;
use memory_values::*;

pub struct DrawData<'a> {
    pub ly: u8,
    pub bgp: u8,
    pub scy: u8,
    lcdc: u8,
    pub memory: &'a Memory,
}

impl<'a> DrawData<'a> {
    pub fn new(regs: &'a mut LCDRegisters) -> DrawData<'a> {
        let ly = regs.get_ly();
        let bgp = regs.get_bgp();
        let scy = regs.get_scy();
        let lcdc = regs.get_lcdc();
        let memory = &regs.memory;

        DrawData { ly, bgp, scy, lcdc, memory }
    }

    pub fn get_bg_tilemap_display_select(&self) -> u16 {
        if self.lcdc.get_bit(3) {
            TILE_MAP_2
        } else {
            TILE_MAP_1
        }
    }

    pub fn get_tile_data_select(&self) -> u16 {
        if self.lcdc.get_bit(4) {
            TILE_DATA_2
        } else {
            TILE_DATA_1
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

    create_getter!(get_lcdc, lcdc, IoRegs::LCDC);

    create_getter!(get_ly, ly, IoRegs::LY);
    create_setter!(set_ly, ly, IoRegs::LY);

    create_getter!(get_lyc, lyc, IoRegs::LYC);

    create_getter!(get_scy, scy, IoRegs::SCY);

    create_getter!(get_bgp, bgp, IoRegs::BGP);

    create_getter!(get_stat, stat, IoRegs::STAT);
    create_setter!(set_stat, stat, IoRegs::STAT);

    pub fn check_enabled(&mut self) -> bool {
        self.get_lcdc().get_bit(7)
    }

    pub fn set_interrupt_bit(&mut self) {
        let if_reg = self.memory.get_io(IoRegs::IF).set_bit(0);
        self.memory.set_io(IoRegs::IF, if_reg);
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
