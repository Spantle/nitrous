#![allow(dead_code)]

use crate::nds::Bits;

pub struct KeyInput(u16);

impl Default for KeyInput {
    fn default() -> Self {
        Self(0b1111111111) // 10 bits
    }
}

impl From<u16> for KeyInput {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl KeyInput {
    const BUTTON_A_OFFSET: u16 = 0;
    const BUTTON_B_OFFSET: u16 = 1;
    const BUTTON_SELECT_OFFSET: u16 = 2;
    const BUTTON_START_OFFSET: u16 = 3;
    const BUTTON_RIGHT_OFFSET: u16 = 4;
    const BUTTON_LEFT_OFFSET: u16 = 5;
    const BUTTON_UP_OFFSET: u16 = 6;
    const BUTTON_DOWN_OFFSET: u16 = 7;
    const BUTTON_R_OFFSET: u16 = 8;
    const BUTTON_L_OFFSET: u16 = 9;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn get_button_a(&self) -> bool {
        self.0.get_bit(Self::BUTTON_A_OFFSET)
    }

    pub fn set_button_a(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_A_OFFSET, released);
    }

    pub fn get_button_b(&self) -> bool {
        self.0.get_bit(Self::BUTTON_B_OFFSET)
    }

    pub fn set_button_b(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_B_OFFSET, released);
    }

    pub fn get_button_select(&self) -> bool {
        self.0.get_bit(Self::BUTTON_SELECT_OFFSET)
    }

    pub fn set_button_select(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_SELECT_OFFSET, released);
    }

    pub fn get_button_start(&self) -> bool {
        self.0.get_bit(Self::BUTTON_START_OFFSET)
    }

    pub fn set_button_start(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_START_OFFSET, released);
    }

    pub fn get_button_right(&self) -> bool {
        self.0.get_bit(Self::BUTTON_RIGHT_OFFSET)
    }

    pub fn set_button_right(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_RIGHT_OFFSET, released);
    }

    pub fn get_button_left(&self) -> bool {
        self.0.get_bit(Self::BUTTON_LEFT_OFFSET)
    }

    pub fn set_button_left(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_LEFT_OFFSET, released);
    }

    pub fn get_button_up(&self) -> bool {
        self.0.get_bit(Self::BUTTON_UP_OFFSET)
    }

    pub fn set_button_up(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_UP_OFFSET, released);
    }

    pub fn get_button_down(&self) -> bool {
        self.0.get_bit(Self::BUTTON_DOWN_OFFSET)
    }

    pub fn set_button_down(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_DOWN_OFFSET, released);
    }

    pub fn get_button_r(&self) -> bool {
        self.0.get_bit(Self::BUTTON_R_OFFSET)
    }

    pub fn set_button_r(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_R_OFFSET, released);
    }

    pub fn get_button_l(&self) -> bool {
        self.0.get_bit(Self::BUTTON_L_OFFSET)
    }

    pub fn set_button_l(&mut self, released: bool) {
        self.0.set_bit(Self::BUTTON_L_OFFSET, released);
    }
}
