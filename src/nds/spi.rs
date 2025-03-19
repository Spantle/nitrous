use super::{logger, shared::Shared, Bits};

// been a while since i've worked on this, this is not my greatest code but it needs to be done asap
// also touchscreen stuff was mostly borrowed from CorgiDS
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Spi {
    pub cnt: SpiControl,
    pub data: u16,

    control_byte: u8,
    data_pos: u8,
    output_coords: u16,
}

impl Spi {
    pub fn write(&mut self, shared: &Shared, input: u16) {
        if !self.cnt.get_enabled() {
            return;
        }

        self.data = match self.cnt.get_device() {
            0 => {
                logger::warn_once(logger::LogSource::Spi, "powerman not implemented");
                0
            }
            1 => {
                logger::warn_once(logger::LogSource::Spi, "firmware not implemented");
                0
            }
            2 => {
                let data = match self.data_pos {
                    0 => (self.output_coords >> 5) & 0xFF,
                    1 => (self.output_coords << 3) & 0xFF,
                    _ => 0,
                };

                if input.get_bit(7) {
                    self.control_byte = input as u8; // fuck it
                    self.data_pos = 0;

                    // let start_bit = input.get_bit(7);
                    // if start_bit {
                    let channel = input.get_bits(4, 6);
                    self.output_coords = match channel {
                        1 => ((shared.touchscreen_point.1 as u32) << 4) as u16, // touchscreen Y
                        5 => ((shared.touchscreen_point.0 as u32) << 4) as u16, // touchscreen X
                        _ => 0,
                    };

                    let conversion_mode = input.get_bit(3);
                    if conversion_mode {
                        self.output_coords &= 0x0FF0;
                    }
                } else {
                    self.data_pos += 1;
                };

                data
            }
            3 => {
                logger::warn_once(logger::LogSource::Spi, "reserved device implemented");
                0
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct SpiControl(u16);

impl SpiControl {
    // const BAUDRATE_START: u16 = 0;
    // const BAUDRATE_END: u16 = 1;

    const BUSY_OFFSET: u16 = 7;
    const DEVICE_START: u16 = 8;
    const DEVICE_END: u16 = 9;

    // const CHIPSET_HOLD_OFFSET: u16 = 11;

    // const IRQ_OFFSET: u16 = 14;
    const ENABLE_OFFSET: u16 = 15;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, value: u16) {
        let busy = self.0.get_bit(Self::BUSY_OFFSET);
        self.0 = value;
        self.0.set_bit(Self::BUSY_OFFSET, busy);
    }

    pub fn get_device(&self) -> u16 {
        self.0.get_bits(Self::DEVICE_START, Self::DEVICE_END)
    }

    pub fn get_enabled(&self) -> bool {
        self.0.get_bit(Self::ENABLE_OFFSET)
    }
}
