macro_rules! create_get_set {
    ( $get_name: ident, $set_name:ident , $high:ident , $low:ident ) => {
        pub fn $get_name (&self) -> u16 {
            (self.$high as u16) << 8 | (self.$low as u16)
        }

        pub fn $set_name (&mut self, value: u16) {
            self.$high = (value >> 8) as u8;
            self.$low = (value & 0xff) as u8;
        }
    };
}

macro_rules! create_get_set_flag {
    ( $flag:ident , $set_name:ident , $mask:expr ) => {
        pub fn $set_name(&mut self, value: bool) {
            if value {
                self.f |= $mask;
            } else {
                self.f &= !$mask;
            }
        }

        pub fn $flag(&self) -> bool {
            self.f & $mask != 0
        }
    };
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0,
            sp: 0,
            pc: 0,
        }
    }

    // create_get_set!(af, set_af, a, f);
    // create_get_set!(bc, set_bc, b, c);
    // create_get_set!(de, set_de, d, e);
    create_get_set!(hl, set_hl, h, l);

    // get value of, and then decrement, the hl register
    pub fn hld(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl - 1);
        hl
    }

    pub fn clear_flags(&mut self) {
        self.f = 0;
    }

    create_get_set_flag!(flagz, set_flagz, 0b1000_0000);
    // create_get_set_flag!(flagn, set_flagn, 0b0100_0000);
    create_get_set_flag!(flagh, set_flagh, 0b0010_0000);
    // create_get_set_flag!(flagc, set_flagc, 0b0001_0000);
}
