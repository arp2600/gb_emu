use bit_ops::BitGetSet;
use std::collections::VecDeque;
use std::default::Default;

#[derive(Debug)]
pub enum AudioAction {
    SetFrequency(u8, f32),
    RestartSound(u8),
}

#[derive(Default)]
pub struct SoundRegisters {
    nr51: u8,
    channel1: Channel2,
    channel2: Channel2,
    channel3: Channel2,
    pub actions: VecDeque<AudioAction>,
}

enum EnvelopeDirection {
    Decrease,
    Increase,
}

impl Default for EnvelopeDirection {
    fn default() -> EnvelopeDirection {
        EnvelopeDirection::Decrease
    }
}

#[derive(Default)]
struct Channel2 {
    frequency_data: u16,
    frequency: f32,
    duty_cycle: u8,
    t1: u8,
    sound_length: f32,
    use_length: bool,
    envelope_start_value: u8,
    envelope_direction: EnvelopeDirection,
    envelope_sweep_num: u8,
}

impl SoundRegisters {
    pub fn new() -> SoundRegisters {
        Default::default()
    }

    pub fn set_nr52(&mut self, value: u8) {
        if value.get_bit(7) {
            eprintln!("turning on sound");
        } else {
            // Turning off the sound clears all sound registers
            eprintln!("warning: turning off sound is unimplemented");
        }
    }

    pub fn set_nr51(&mut self, value: u8) {
        if self.nr51 != value {
            self.nr51 = value;
            eprintln!("setting nr51 to {:#04x}", value);
            for i in 0..4 {
                if value.get_bit(i) {
                    eprintln!("outputing sound {} to SO1", i + 1);
                }
                if value.get_bit(4 + i) {
                    eprintln!("outputing sound {} to SO2", i + 1);
                }
            }
        }
    }

    pub fn set_nr50(&mut self, value: u8) {
        if value.get_bit(7) {
            eprintln!("outputing vin to SO2");
        }
        if value.get_bit(3) {
            eprintln!("outputing vin to SO1");
        }
        let so2_level = (value >> 4) & 0b11;
        eprintln!("so2 level {}", so2_level);
        let so1_level = value & 0b11;
        eprintln!("so1 level {}", so1_level);
    }

    pub fn set_nr10(&mut self, _value: u8) {}

    pub fn set_nr11(&mut self, _value: u8) {}

    pub fn set_nr12(&mut self, _value: u8) {}

    pub fn set_nr13(&mut self, value: u8) {
        let c = &mut self.channel1;

        c.frequency_data = {
            let x = u16::from(value);
            (c.frequency_data & 0xff00) | x
        };
        c.frequency = 131072.0 / (2048.0 - c.frequency_data as f32);
        self.actions
            .push_back(AudioAction::SetFrequency(1, c.frequency));
    }

    pub fn set_nr14(&mut self, value: u8) {
        let c = &mut self.channel1;

        if value.get_bit(7) {
            self.actions.push_back(AudioAction::RestartSound(1));
        }
        c.use_length = value.get_bit(6);

        c.frequency_data = {
            let x = u16::from(value & 0b111) << 8;
            (c.frequency_data & 0x00ff) | x
        };
        c.frequency = 131072.0 / (2048.0 - c.frequency_data as f32);
        self.actions
            .push_back(AudioAction::SetFrequency(1, c.frequency));
    }

    pub fn set_nr21(&mut self, value: u8) {
        let c = &mut self.channel2;

        c.duty_cycle = value >> 6;
        c.t1 = value & 0b1_1111;
        c.sound_length = (64.0 - c.t1 as f32) * (1.0 / 256.0);
    }

    pub fn set_nr22(&mut self, value: u8) {
        let c = &mut self.channel2;

        c.envelope_start_value = value >> 4;
        c.envelope_direction = if value.get_bit(3) {
            EnvelopeDirection::Increase
        } else {
            EnvelopeDirection::Decrease
        };
        c.envelope_sweep_num = value & 0b111;
    }

    pub fn set_nr23(&mut self, value: u8) {
        let c = &mut self.channel2;

        c.frequency_data = {
            let x = u16::from(value);
            (c.frequency_data & 0xff00) | x
        };
        c.frequency = 131072.0 / (2048.0 - c.frequency_data as f32);
        self.actions
            .push_back(AudioAction::SetFrequency(2, c.frequency));
    }

    pub fn set_nr24(&mut self, value: u8) {
        let c = &mut self.channel2;

        if value.get_bit(7) {
            self.actions.push_back(AudioAction::RestartSound(2));
        }
        c.use_length = value.get_bit(6);

        c.frequency_data = {
            let x = u16::from(value & 0b111) << 8;
            (c.frequency_data & 0x00ff) | x
        };
        c.frequency = 131072.0 / (2048.0 - c.frequency_data as f32);
        self.actions
            .push_back(AudioAction::SetFrequency(2, c.frequency));
    }

    pub fn set_nr30(&mut self, _value: u8) {}

    pub fn set_nr31(&mut self, _value: u8) {}

    pub fn set_nr32(&mut self, _value: u8) {}

    pub fn set_nr33(&mut self, value: u8) {
        let c = &mut self.channel3;

        c.frequency_data = {
            let x = u16::from(value);
            (c.frequency_data & 0xff00) | x
        };
        c.frequency = 131072.0 / (2048.0 - c.frequency_data as f32);
        self.actions
            .push_back(AudioAction::SetFrequency(3, c.frequency));
    }

    pub fn set_nr34(&mut self, value: u8) {
        let c = &mut self.channel3;

        if value.get_bit(7) {
            self.actions.push_back(AudioAction::RestartSound(3));
        }
        c.use_length = value.get_bit(6);

        c.frequency_data = {
            let x = u16::from(value & 0b111) << 8;
            (c.frequency_data & 0x00ff) | x
        };
        c.frequency = 131072.0 / (2048.0 - c.frequency_data as f32);
        self.actions
            .push_back(AudioAction::SetFrequency(3, c.frequency));
    }

    pub fn set_nr41(&mut self, _value: u8) {}

    pub fn set_nr42(&mut self, _value: u8) {}

    pub fn set_nr43(&mut self, _value: u8) {}

    pub fn set_nr44(&mut self, _value: u8) {}
}
