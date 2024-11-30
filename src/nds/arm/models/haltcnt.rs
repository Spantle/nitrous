use bitflags::bitflags;

use super::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct HaltCnt(u8);

impl From<u8> for HaltCnt {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl HaltCnt {
    const POWER_DOWN_MODE_START: u8 = 6;
    const POWER_DOWN_MODE_END: u8 = 7;

    pub fn set(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn get_power_down_mode(&self) -> PowerDownMode {
        PowerDownMode::from_bits_truncate(
            self.0
                .get_bits(Self::POWER_DOWN_MODE_START, Self::POWER_DOWN_MODE_END),
        )
    }
}

bitflags! {
    #[derive(PartialEq)]
    pub struct PowerDownMode: u8 {
        const NO_FUNCTION = 0;
        const ENTER_GBA_MODE = 1;
        const HALT = 2;
        const SLEEP = 3;
    }
}
