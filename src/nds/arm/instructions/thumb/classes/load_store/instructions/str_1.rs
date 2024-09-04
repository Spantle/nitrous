use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// STR (1)
pub fn str_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STR");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let immed_5 = ctx.inst.get_word(6, 10);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_word_end_arg(immed_5, Some(", "));
    ctx.dis.push_str_end_arg(" * 4", None);
    ctx.dis.push_str_end_arg("", Some("]"));

    let rd = ctx.arm.r()[rd];
    let rn = ctx.arm.r()[rn];
    let address = rn + (immed_5 * 4);
    // technically if address bits 0-1 aren't 0 then it's UNPREDICTABLE
    ctx.arm.write_word(ctx.bus, ctx.shared, address, rd);

    1 // TODO: this is wrong
}
