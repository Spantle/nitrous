use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// SWPB
pub fn swpb(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SWP");
    ctx.dis.set_inst_suffix("B");

    let rm = ctx.inst.get_byte(0, 3);
    let rd = ctx.inst.get_byte(12, 15);
    let rn = ctx.inst.get_byte(16, 19);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_str_end_arg("", Some("]"));

    let rn = ctx.arm.er(rn);
    let temp = ctx.arm.read_byte(ctx.bus, ctx.shared, ctx.dma, rn);

    ctx.arm.write_byte(
        ctx.bus,
        ctx.shared,
        ctx.dma,
        rn,
        ctx.arm.er(rm).get_bits(0, 7) as u8,
    );
    ctx.arm.set_r(rd, temp as u32);

    1 // TODO: this is wrong
}
