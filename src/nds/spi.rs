use super::{logger, Bits};

// been a while since i've worked on this, this is not my greatest code but it needs to be done asap

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Spi {
    pub spicnt: SpiControl,

    data: u16,
    data_position: u8,
}

impl Spi {
    pub fn write(&mut self, input: u8) {
        if !self.spicnt.get_enabled() {
            return;
        }

        match self.spicnt.get_device() {
            0 => logger::warn_once(logger::LogSource::Spi, "firmware not implemented"),
            1 => {
                let start_bit = input.get_bit(7);
                if start_bit {
                    let channel = input.get_bits(4, 6);
                    self.data = match channel {
                        1 => 0, // touchscreen Y
                        5 => 0, // touchscreen X
                        _ => 0,
                    };

                    let conversion_mode = input.get_bit(3);
                    if conversion_mode {
                        self.data &= 0x0FF0;
                    }
                }
            }
            2 => logger::warn_once(logger::LogSource::Spi, "powerman not implemented"),
            3 => logger::warn_once(logger::LogSource::Spi, "512KHz not implemented"),
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
