pub trait BitGetSet {
    type Item;
    fn set_bit(&self, bit: u8) -> Self::Item;
    fn reset_bit(&self, bit: u8) -> Self::Item;
    fn get_bit(&self, bit: u8) -> bool;
}

impl BitGetSet for u8 {
    type Item = u8;

    fn get_bit(&self, bit: u8) -> bool {
        self & (1 << bit) != 0
    }

    fn set_bit(&self, bit: u8) -> Self::Item {
        self | (1 << bit)
    }

    fn reset_bit(&self, bit: u8) -> Self::Item {
        self & !(1 << bit)
    }
}
