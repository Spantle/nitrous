mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::cpu::arm9::models::Bits;

fn sign_extend_24_to_32(value: u32) -> i32 {
    let sign_bit = value.get_bit(23);

    let extended_value = if sign_bit {
        value | 0xFF000000
    } else {
        value & 0x00FFFFFF
    };

    extended_value as i32
}
