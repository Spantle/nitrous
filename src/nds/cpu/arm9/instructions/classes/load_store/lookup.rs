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

    let address = if inst.post_indexing {
        let address = arm9.eru(inst.first_source_register);
        if inst.is_add {
            arm9.r[inst.first_source_register] += inst.addressing_mode;
        } else {
            arm9.r[inst.first_source_register] -= inst.addressing_mode;
        }
        address
    } else if inst.is_add {
        arm9.eru(inst.first_source_register) + inst.addressing_mode
    } else {
        arm9.eru(inst.first_source_register) - inst.addressing_mode
    };

    if inst.w {
        arm9.r[inst.first_source_register] = address;
    };

    // there's also some unpredictability if it's "Scaled register pre-indexed" and Rn and Rm are the same
    // i'm sure it's fine
    if !inst.is_unsigned_byte {
        if inst.is_load {
            return instructions::ldr(inst, address, arm9, bus);
        }
    }

    logger::warn(format!(
        "unknown load/store inst {} {} {} {} {}",
        inst.post_indexing, inst.is_add, inst.is_unsigned_byte, inst.w, inst.is_load
    ));
    1
}
