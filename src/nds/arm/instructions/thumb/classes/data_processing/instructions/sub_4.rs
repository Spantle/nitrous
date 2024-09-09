use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// SUB (4)
pub fn sub_4(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SUB");

    let immed_7 = ctx.inst.get_word(0, 6);
    ctx.dis.push_str_arg("SP");
    ctx.dis.push_word_end_arg(immed_7, None);
    ctx.dis.push_str_end_arg(" * 4", None);

    let result = ctx.arm.r()[13] - (immed_7 << 2);
    ctx.arm.set_r(13, result);

    1 // TODO: this is wrong
}
