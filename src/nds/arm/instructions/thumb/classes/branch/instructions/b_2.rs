use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// B (2)
pub fn b_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("B");

    let pc = (ctx.arm.r()[15] + 4) as i32;
    let signed_immed_11 = ctx.inst.get_word(0, 10).sign_extend(11);
    let signed_immed_11 = (pc + (signed_immed_11 << 1)) as u32;
    ctx.dis.push_word_arg(signed_immed_11);

    ctx.arm.set_r(15, signed_immed_11);

    1 // TODO: this is wrong
}
