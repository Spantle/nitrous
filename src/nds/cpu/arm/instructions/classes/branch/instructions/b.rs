use crate::nds::cpu::arm::{
    arm::ArmTrait,
    instructions::classes::branch::sign_extend_24_to_32,
    models::{Context, ContextTrait, DisassemblyTrait, Instruction},
};

// B, BL
pub fn b<const L: bool>(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    if L {
        arm.set_r(14, arm.r()[15].wrapping_add(4));
    }

    let signed_immed_24 = inst.get_word(0, 23);
    let signed_immed_24 = sign_extend_24_to_32(signed_immed_24) << 2;
    let result = (arm.er(15) as i32).wrapping_add(signed_immed_24) as u32; // TODO: probably not the best conversion?
    ctx.dis.push_word_arg(result);
    arm.set_r(15, result);

    3
}
