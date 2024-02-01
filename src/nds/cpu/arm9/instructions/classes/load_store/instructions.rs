use crate::nds::cpu::{arm9::Arm9, bus::Bus};

use super::LoadStoreInstruction;

// LDR
#[inline(always)]
pub fn ldr(inst: LoadStoreInstruction, address: u32, arm9: &mut Arm9, bus: &mut Bus) -> u32 {
    let bits = address & 0b11; // i have no idea what to call this
    let mut cycles = 1 + (bits != 0) as u32;

    // If register 15 is specified for <Rd>, address[1:0] must be 0b00. If not, the result is UNPREDICTABLE.
    // let value = match bits {
    //     0b00 => bus.read_word(address),
    //     0b01 => bus.read_word(address).rotate_right(8),
    //     0b10 => bus.read_word(address).rotate_right(16),
    //     0b11 => bus.read_word(address).rotate_right(24),
    //     _ => unreachable!(),
    // };
    let value = bus.read_word(address).rotate_right(bits * 8);

    if inst.destination_register == 15 {
        // note: this is for armv5
        arm9.r[15] = value & 0xFFFFFFFE;
        arm9.cpsr.set_thumb(value & 1 != 0);
        cycles += 4;
    } else {
        arm9.r[inst.destination_register] = value;
    }

    cycles
}

// STR
#[inline(always)]
pub fn str(inst: LoadStoreInstruction, address: u32, arm9: &mut Arm9, bus: &mut Bus) -> u32 {
    bus.write_word(address, arm9.r[inst.destination_register]);

    1
}
