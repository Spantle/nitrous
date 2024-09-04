use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// LDR (4)
pub fn ldr_4(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LDR");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_end_arg("SP", Some("["));
    ctx.dis.push_word_end_arg(immed_8, Some(", "));
    ctx.dis.push_str_end_arg(" * 4", None);
    ctx.dis.push_str_end_arg("", Some("]"));

    let sp = ctx.arm.ert(13);
    let address = sp + (immed_8 * 4);
    ctx.arm
        .set_r(rd, ctx.arm.read_word(ctx.bus, ctx.shared, address));

    1 // TODO: this is wrong
}
