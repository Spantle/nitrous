use super::{logger, shared::Shared, Bits};

// been a while since i've worked on this, this is not my greatest code but it needs to be done asap
// also touchscreen+firmware stuff was mostly borrowed from CorgiDS
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Spi {
    pub cnt: SpiControl,
    pub data: u16,

    fw_command_id: FirmwareCommand,
    fw_address: u32,
    fw_total_args: u8,
    fw_status_reg: u8,

    tsc_control_byte: u8,
    tsc_data_pos: u8,
    tsc_output_coords: u16,
}

impl Spi {
    pub fn write(&mut self, firmware: &[u8], shared: &Shared, input: u16) {
        if !self.cnt.get_enabled() {
            return;
        }

        self.data = match self.cnt.get_device() {
            0 => {
                logger::warn_once(logger::LogSource::Spi, "powerman not implemented");
                0
            }
            1 => {
                self.fw_total_args = self.fw_total_args.wrapping_add(1);

                let data = match self.fw_command_id {
                    FirmwareCommand::ReadStream => {
                        if self.fw_total_args < 5 {
                            self.fw_address <<= 8;
                            self.fw_address |= input as u32;
                            input
                        } else {
                            self.fw_address += 1;
                            firmware[(self.fw_address as usize - 1) & (1024 * 256 - 1)] as u16
                        }
                    }
                    FirmwareCommand::ReadStatusReg => self.fw_status_reg as u16,
                    _ => {
                        match input {
                            0x03 => self.fw_command_id = FirmwareCommand::ReadStream,
                            0x04 => self.fw_status_reg &= !0x1,
                            0x05 => self.fw_command_id = FirmwareCommand::ReadStatusReg,
                            0x06 => self.fw_status_reg |= 0x1,
                            _ => logger::error(
                                logger::LogSource::Spi,
                                format!("unknown firmware command {}", input),
                            ),
                        }
                        input
                    }
                };

                if !self.cnt.get_chipset_hold() {
                    self.fw_command_id = FirmwareCommand::None;
                    self.fw_address = 0;
                    self.fw_total_args = 0;
                };

                data
            }
            2 => {
                let data = match self.tsc_data_pos {
                    0 => (self.tsc_output_coords >> 5) & 0xFF,
                    1 => (self.tsc_output_coords << 3) & 0xFF,
                    _ => 0,
                };

                if input.get_bit(7) {
                    self.tsc_control_byte = input as u8; // screw it
                    self.tsc_data_pos = 0;

                    let channel = input.get_bits(4, 6);
                    self.tsc_output_coords = match channel {
                        1 => ((shared.touchscreen_point.1 as u32) << 4) as u16, // touchscreen Y
                        5 => ((shared.touchscreen_point.0 as u32) << 4) as u16, // touchscreen X
                        _ => 0,
                    };

                    let conversion_mode = input.get_bit(3);
                    if conversion_mode {
                        self.tsc_output_coords &= 0x0FF0;
                    }
                } else {
                    self.tsc_data_pos += 1;
                };

                data
            }
            3 => {
                logger::error_once(logger::LogSource::Spi, "reserved device not implemented");
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

    const CHIPSET_HOLD_OFFSET: u16 = 11;

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

    pub fn get_chipset_hold(&self) -> bool {
        self.0.get_bit(Self::CHIPSET_HOLD_OFFSET)
    }

    pub fn get_enabled(&self) -> bool {
        self.0.get_bit(Self::ENABLE_OFFSET)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
enum FirmwareCommand {
    #[default]
    None,
    ReadStatusReg,
    ReadStream,
}
