use bit_ops::BitGetSet;
use std::collections::VecDeque;
use std::default::Default;

#[derive(Debug)]
pub enum AudioAction {
    SetFrequency(u8, f32),
    SetAmplitude(u8, f32),
    RestartSound(u8),
    SetPulseWidth(u8, f32),
}

#[derive(Default)]
pub struct SoundRegisters {
    // nr10: u8,
    // nr11: u8,
    // nr12: u8,
    // nr13: u8,
    // nr14: u8,
    // nr21: u8,
    // nr22: u8,
    // nr23: u8,
    // nr24: u8,
    // nr30: u8,
    // nr31: u8,
    // nr32: u8,
    // nr33: u8,
    // nr34: u8,
    // nr41: u8,
    // nr42: u8,
    // nr43: u8,
    // nr44: u8,
    // nr50: u8,
    nr51: u8,
    // nr52: u8,
    frequencies: [u16; 4],
    pub actions: VecDeque<AudioAction>,
}

#[derive(Debug)]
enum EnvelopeDirection {
    Decrease,
    Increase,
}

impl Default for EnvelopeDirection {
    fn default() -> EnvelopeDirection {
        EnvelopeDirection::Decrease
    }
}

impl SoundRegisters {
    pub fn new() -> SoundRegisters {
        SoundRegisters {
            ..Default::default()
        }
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

    pub fn set_nr12(&mut self, value: u8) {
        self.set_channel_volume_envelope_register(0, value);
    }

    pub fn set_nr13(&mut self, value: u8) {
        self.set_frequency_low_data(value, 1);
    }

    pub fn set_nr14(&mut self, value: u8) {
        self.set_frequency_high_data(value, 1);
    }

    pub fn set_nr21(&mut self, value: u8) {
        let duty_cycle = value >> 6;
        let pw = match duty_cycle {
            0 => 12.5,
            1 => 25.0,
            2 => 50.0,
            3 => 75.0,
            _ => unreachable!(),
        };
        self.actions.push_back(AudioAction::SetPulseWidth(2, pw));

        let t1 = value & 0b1_1111;
        let sound_length = (64.0 - t1 as f32) * (1.0 / 256.0);
        println!("c2 sound length = {}", sound_length);
    }

    pub fn set_nr22(&mut self, value: u8) {
        self.set_channel_volume_envelope_register(1, value);
    }

    pub fn set_nr23(&mut self, value: u8) {
        self.set_frequency_low_data(value, 2);
    }

    pub fn set_nr24(&mut self, value: u8) {
        self.set_frequency_high_data(value, 2);
    }

    pub fn set_nr30(&mut self, _value: u8) {}

    pub fn set_nr31(&mut self, _value: u8) {}

    pub fn set_nr32(&mut self, _value: u8) {}

    pub fn set_nr33(&mut self, value: u8) {
        self.set_frequency_low_data(value, 3);
    }

    pub fn set_nr34(&mut self, value: u8) {
        self.set_frequency_high_data(value, 3);
    }

    pub fn set_nr41(&mut self, _value: u8) {}

    pub fn set_nr42(&mut self, _value: u8) {}

    pub fn set_nr43(&mut self, _value: u8) {}

    pub fn set_nr44(&mut self, _value: u8) {}

    fn set_channel_volume_envelope_register(&mut self, chan_index: usize, value: u8) {
        let envelope_start_value = value >> 4;
        let amp = envelope_start_value as f32 / 16.0;
        self.actions
            .push_back(AudioAction::SetAmplitude((chan_index + 1) as u8, amp));

        let envelope_direction = if value.get_bit(3) {
            EnvelopeDirection::Increase
        } else {
            EnvelopeDirection::Decrease
        };
        println!(
            "c{} envelope_direction = {:?}",
            chan_index + 1,
            envelope_direction
        );
        let envelope_sweep_num = value & 0b111;
        println!(
            "c{} envelope_sweep_num = {}",
            chan_index + 1,
            envelope_sweep_num
        );
    }

    fn set_frequency_low_data(&mut self, value: u8, chan_num: u8) {
        let chan_index = usize::from(chan_num - 1);
        let frequency_data = &mut self.frequencies[chan_index];
        set_frequency_register_low_bits(frequency_data, value);
        let frequency = calculate_frequency(*frequency_data);

        self.actions
            .push_back(AudioAction::SetFrequency(chan_num, frequency));
    }

    fn set_frequency_high_data(&mut self, value: u8, chan_num: u8) {
        if value.get_bit(7) {
            self.actions.push_back(AudioAction::RestartSound(chan_num));
        }
        let use_length = value.get_bit(6);
        println!("c{} use_length = {}", chan_num, use_length);

        let chan_index = usize::from(chan_num - 1);
        let frequency_data = &mut self.frequencies[chan_index];
        set_frequency_register_high_bits(frequency_data, value);
        let frequency = calculate_frequency(*frequency_data);
        self.actions
            .push_back(AudioAction::SetFrequency(chan_num, frequency));
    }
}

fn set_frequency_register_low_bits(register: &mut u16, value: u8) {
    *register = {
        let x = u16::from(value);
        (*register & 0xff00) | x
    };
}

fn set_frequency_register_high_bits(register: &mut u16, value: u8) {
    *register = {
        let x = u16::from(value & 0b111) << 8;
        (*register & 0x00ff) | x
    };
}

fn calculate_frequency(register: u16) -> f32 {
    131072.0 / (2048.0 - register as f32)
}
