use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// STR (2)
pub fn str_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STR");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let rm = ctx.inst.get_byte(6, 8);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_reg_end_arg(rm, Some(", "));
    ctx.dis.push_str_end_arg("", Some("]"));

    // technically if address bits 0-1 aren't 0 then it's UNPREDICTABLE
    let address = ctx.arm.r()[rn].wrapping_add(ctx.arm.r()[rm]);
    let rd = ctx.arm.r()[rd];
    ctx.arm.write_word(ctx.bus, ctx.shared, ctx.dma,address, rd);

    1 // TODO: this is wrong
}
