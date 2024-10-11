use crate::nds::Bits;

pub struct ExtKeyIn(u16);

impl Default for ExtKeyIn {
    fn default() -> Self {
        Self(0b1111111) // 7 bits
    }
}

impl From<u16> for ExtKeyIn {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl ExtKeyIn {
    const BUTTON_X_OFFSET: u16 = 0;
    const BUTTON_Y_OFFSET: u16 = 1;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn get_button_x(&self) -> bool {
        self.0.get_bit(Self::BUTTON_X_OFFSET)
    }

    pub fn set_button_x(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_X_OFFSET, released);
    }

    pub fn get_button_y(&self) -> bool {
        self.0.get_bit(Self::BUTTON_Y_OFFSET)
    }

    pub fn set_button_y(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_Y_OFFSET, released);
    }
}
