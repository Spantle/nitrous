use crate::nds::cpu::arm9::{
    arm9::Arm9Trait,
    instructions::classes::branch::sign_extend_24_to_32,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

// B, BL
pub fn b<const L: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let (arm9, inst) = (&mut ctx.arm9, &ctx.inst);
    if L {
        arm9.r()[14] = arm9.r()[15].wrapping_add(4);
    }

    let signed_immed_24 = inst.get_word(0, 23);
    let signed_immed_24 = sign_extend_24_to_32(signed_immed_24) << 2;
    let result = (arm9.er(15) as i32).wrapping_add(signed_immed_24) as u32; // TODO: probably not the best conversion?
    ctx.dis.push_word_arg(result);
    arm9.r()[15] = result;

    3
}
