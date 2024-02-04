use crate::nds::cpu::arm9::{instructions::models::Instruction, Arm9};

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, inst: Instruction, arm9: &mut Arm9) -> u32 {
    let l = (inst_set >> 8) & 1 != 0; // L bit

    if l {
        instructions::b::<true>(inst, arm9)
    } else {
        instructions::b::<false>(inst, arm9)
    }
}
