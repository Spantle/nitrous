use crate::nds::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct AuxSpiCnt(u16);

impl AuxSpiCnt {
    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn set(&mut self, data: u16) {
        let bit7 = self.0.get_bit(7);
        self.0 = data;
        self.0.set_bit(7, bit7);
    }

    pub fn set_hi(&mut self, data: u16) {
        self.0.set_bits(8, 15, data);
    }
}
