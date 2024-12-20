use crate::nds::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct BldCnt(pub u16);

impl From<u16> for BldCnt {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl BldCnt {
    const COLOR_SPECIAL_EFFECT_START: u16 = 6;
    const COLOR_SPECIAL_EFFECT_END: u16 = 7;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn get_first_target_pixel(&self, target: u16) -> bool {
        self.0.get_bit(target)
    }

    pub fn get_color_special_effect(&self) -> ColorSpecialEffect {
        let value = self.0.get_bits(
            Self::COLOR_SPECIAL_EFFECT_START,
            Self::COLOR_SPECIAL_EFFECT_END,
        );
        match value {
            0 => ColorSpecialEffect::None,
            1 => ColorSpecialEffect::AlphaBlending,
            2 => ColorSpecialEffect::BrightnessIncrease,
            3 => ColorSpecialEffect::BrightnessDecrease,
            _ => unreachable!(),
        }
    }

    pub fn get_second_target_pixel(&self, target: u16) -> bool {
        self.0.get_bit(target + 8)
    }
}

pub enum ColorSpecialEffect {
    None = 0,
    AlphaBlending = 1,
    BrightnessIncrease = 2,
    BrightnessDecrease = 3,
}

impl ColorSpecialEffect {
    pub fn alpha_blend(first: u16, second: u16, eva: f32, evb: f32) -> u16 {
        if !first.get_bit(15) {
            return second;
        }

        let r = 31_f32.min(first.get_bits(0, 4) as f32 * evb + second.get_bits(0, 4) as f32 * eva)
            as u16;
        let g = 31_f32.min(first.get_bits(5, 9) as f32 * evb + second.get_bits(5, 9) as f32 * eva)
            as u16;
        let b = 31_f32
            .min(first.get_bits(10, 14) as f32 * evb + second.get_bits(10, 14) as f32 * eva)
            as u16;
        let mut result = 0;
        result.set_bits(0, 4, r);
        result.set_bits(5, 9, g);
        result.set_bits(10, 14, b);
        result.set_bit(15, false);
        result
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct BldAlpha(u8);

impl From<u8> for BldAlpha {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl BldAlpha {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn ev(&self) -> f32 {
        let ev = self.0.get_bits(0, 4) as f32;
        let n = 16_f32.min(ev);
        n / 16.0
    }
}
