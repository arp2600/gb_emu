pub struct PixelIterator {
    i: u8,
    low: u8,
    high: u8,
}

impl PixelIterator {
    pub fn new(value: u16) -> PixelIterator {
        let low = value as u8;
        let high = (value >> 8) as u8;
        PixelIterator { i: 0, low, high }
    }
}

impl Iterator for PixelIterator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 8 {
            let low = (self.low >> 7) & 0b1;
            let high = (self.high >> 6) & 0b10;
            self.low <<= 1;
            self.high <<= 1;
            self.i += 1;
            Some((low | high) as u8)
        } else {
            None
        }
    }
}
