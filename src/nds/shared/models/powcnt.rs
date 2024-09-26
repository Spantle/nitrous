use crate::nds::Bits;

#[derive(Default)]
pub struct PowCnt1(u32);

impl From<u32> for PowCnt1 {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl PowCnt1 {
    const DISPLAY_SWAP_OFFSET: u32 = 15;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn get_display_swap(&self) -> bool {
        self.0.get_bit(Self::DISPLAY_SWAP_OFFSET)
    }
}
