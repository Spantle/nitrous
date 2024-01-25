use crate::nds::{
    cpu::{
        arm9::{instructions::models::Instruction, Arm9},
        bus::Bus,
    },
    logger,
};

use super::{instructions, LoadStoreInstruction};

#[inline(always)]
pub fn lookup<const INST_SET: u16, const IS_REGISTER: bool>(
    o_inst: Instruction,
    arm9: &mut Arm9,
    bus: &mut Bus,
) -> u32 {
    let inst = LoadStoreInstruction::new::<IS_REGISTER>(&*arm9, o_inst);
    let post_indexing = INST_SET >> 8 & 1 == 0; // P: technically 0 but we've flipped it since 1 is "offset"/"pre-indexed" addressing
    let is_add = INST_SET >> 7 & 1 == 1; // U
    let is_unsigned_byte = INST_SET >> 6 & 1 == 1; // B
    let w = INST_SET >> 5 & 1 == 1; // W
    let is_load = INST_SET >> 4 & 1 == 1; // L

    let address = if post_indexing {
        let address = arm9.eru(inst.first_source_register);
        if is_add {
            arm9.r[inst.first_source_register] += inst.addressing_mode;
        } else {
            arm9.r[inst.first_source_register] -= inst.addressing_mode;
        }
        address
    } else if is_add {
        arm9.eru(inst.first_source_register) + inst.addressing_mode
    } else {
        arm9.eru(inst.first_source_register) - inst.addressing_mode
    };

    if w {
        arm9.r[inst.first_source_register] = address;
    };

    // there's also some unpredictability if it's "Scaled register pre-indexed" and Rn and Rm are the same
    // i'm sure it's fine
    if !is_unsigned_byte {
        if is_load {
            return instructions::ldr(inst, address, arm9, bus);
        } else {
            return instructions::str(inst, address, arm9, bus);
        }
    }

    logger::warn(format!(
        "unknown load/store inst {} {} {} {} {}",
        post_indexing, is_add, is_unsigned_byte, w, is_load
    ));
    1
}
