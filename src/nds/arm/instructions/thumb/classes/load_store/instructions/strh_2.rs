use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// STRH (2)
pub fn strh_2(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STRH");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let rm = ctx.inst.get_byte(6, 8);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_reg_end_arg(rm, Some(", "));
    ctx.dis.push_str_end_arg("", Some("]"));

    // NOTE: technically it's UNPREDICTABLE if bit 0 of address is not 0
    let address = ctx.arm.r()[rn] + ctx.arm.r()[rm];
    let rd = ctx.arm.r()[rd];
    ctx.arm.write_halfword(
        ctx.bus,
        ctx.shared,
        ctx.dma,
        address,
        rd.get_bits(0, 15) as u16,
    );

    1 // TODO: this is wrong
}
