use memory::Memory;

pub struct LCD {
    enabled: bool,
    frame_start: u64,
}

impl LCD {
    pub fn new() -> LCD {
        LCD {
            enabled: false,
            frame_start: 0,
        }
    }

    pub fn tick(&mut self, memory: &mut Memory, clock: u64) {
        let enabled = memory.get_u8(0xff40) & 0b1000_0000 != 0;
        if self.enabled != enabled {
            self.frame_start = clock;
            self.enabled = enabled;
        }

        if self.enabled {
            let frame_time = (clock - self.frame_start) % (456 * 154);
            let ly = (frame_time / 456) as i32;
            assert!(ly < 154);
            memory.set_u8(0xff44, ly as u8)
        }
    }
}
