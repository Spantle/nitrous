use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// STR (3)
pub fn str_3(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STR");

    let immed_8 = ctx.inst.get_word(0, 7);
    let rd = ctx.inst.get_byte(8, 10);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_end_arg("SP", Some("["));
    ctx.dis.push_word_end_arg(immed_8, Some(", "));
    ctx.dis.push_str_end_arg(" * 4", None);
    ctx.dis.push_str_end_arg("", Some("]"));

    let rd = ctx.arm.r()[rd];
    let sp = ctx.arm.r()[13];
    let address = sp + (immed_8 * 4);
    // technically if address bits 0-1 aren't 0 then it's UNPREDICTABLE
    ctx.arm.write_word(ctx.bus, ctx.shared, ctx.dma,address, rd);

    1 // TODO: this is wrong
}
