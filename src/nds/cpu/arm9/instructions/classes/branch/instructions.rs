use crate::nds::cpu::arm9::models::Context;

use super::sign_extend_24_to_32;

// B, BL
pub fn b<const L: bool>(ctx: Context) -> u32 {
    let (arm9, inst) = (ctx.arm9, ctx.inst);
    if L {
        arm9.r[14] = arm9.r[15].wrapping_add(4);
    }

    let signed_immed_24 = inst.get_word(0, 23);
    arm9.r[15] =
        (arm9.er(15) as i32).wrapping_add(sign_extend_24_to_32(signed_immed_24) << 2) as u32;

    3
}
