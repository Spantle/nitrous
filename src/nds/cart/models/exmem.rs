// TODO: MIRRORING AND WHATNOT
// THESE ARE A BIT LIKE IPCSYNC :(

use crate::nds::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct ExMem(pub u16);

impl ExMem {
    // true: arm9, false: arm7
    pub fn get_nds_slot_access_rights(&self) -> bool {
        self.0.get_bit(11)
    }
}
