use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// LDRB (2)
pub fn ldrb_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LDRB");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let rm = ctx.inst.get_byte(6, 8);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_reg_end_arg(rm, Some(", "));
    ctx.dis.push_str_end_arg("", Some("]"));

    let address = ctx.arm.r()[rn] + ctx.arm.r()[rm];
    ctx.arm
        .set_r(rd, ctx.arm.read_byte(ctx.bus, ctx.shared, ctx.dma,address) as u32);

    1 // TODO: this is wrong
}
