use crate::nds::cpu::arm9::models::Context;

use super::instructions;

#[inline(always)]
pub fn lookup(inst_set: u16, ctx: Context) -> u32 {
    let l = (inst_set >> 8) & 1 != 0; // L bit

    if l {
        instructions::b::<true>(ctx)
    } else {
        instructions::b::<false>(ctx)
    }
}
