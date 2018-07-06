use bit_ops::BitGetSet;

pub struct JoyPad {
    buttons: u8,
    directions: u8,
    // 0x20 for for buttons, 0x10 for directions
    // 0x30 and 0x00 ???
    selection: u8,
}

fn set_bit(x: u8, bit: u8, state: bool) -> u8 {
    if state {
        x | (0b1 << bit)
    } else {
        x & !(0b1 << bit)
    }
}

impl JoyPad {
    pub fn set_a(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 0, !state);
    }

    pub fn set_b(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 1, !state);
    }

    pub fn set_select(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 2, !state);
    }

    pub fn set_start(&mut self, state: bool) {
        self.buttons = set_bit(self.buttons, 3, !state);
    }

    pub fn set_right(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 0, !state);
    }

    pub fn set_left(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 1, !state);
    }

    pub fn set_up(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 2, !state);
    }

    pub fn set_down(&mut self, state: bool) {
        self.directions = set_bit(self.directions, 3, !state);
    }

    pub(super) fn new() -> JoyPad {
        JoyPad {
            buttons: 0x0f,
            directions: 0x0f,
            selection: 0x30,
        }
    }

    pub(super) fn set_u8(&mut self, value: u8) {
        self.selection = value & 0b0011_0000;
    }

    pub(super) fn get_u8(&self) -> u8 {
        let p14 = self.selection.get_bit(4);
        let p15 = self.selection.get_bit(5);

        if !p15 && p14 {
            0b1100_1111 & self.buttons
        } else if p15 && !p14 {
            0b1100_1111 & self.directions
        } else {
            0b1100_1111
        }
    }
}

