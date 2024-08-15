use crate::nds::arm::{
    arm::ArmTrait,
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
};

// ADD (5)
pub fn add_5(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("ADD");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_end_arg("PC", None);
    ctx.dis.push_word_end_arg(immed_8, Some(", "));
    ctx.dis.push_str_end_arg(" * 4", None);

    let pc = ctx.arm.ert(15) & 0xFFFFFFFC;
    let immed_8 = immed_8 << 2;
    let result = pc.wrapping_add(immed_8);

    ctx.arm.set_r(rd, result);
}