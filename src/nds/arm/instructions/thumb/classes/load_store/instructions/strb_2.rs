use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// STRB (2)
pub fn strb_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STRB");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let rm = ctx.inst.get_byte(6, 8);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_reg_end_arg(rm, Some(", "));
    ctx.dis.push_str_end_arg("", Some("]"));

    let address = ctx.arm.r()[rn] + ctx.arm.r()[rm];
    let rd = ctx.arm.r()[rd];
    ctx.arm.write_byte(
        ctx.bus,
        ctx.shared,
        ctx.dma,
        address,
        rd.get_bits(0, 7) as u8,
    );

    1 // TODO: this is wrong
}
