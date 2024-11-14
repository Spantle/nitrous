use crate::nds::{
    arm::{
        instructions::arm::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// SWP
pub fn swp(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("SWP");

    let rm = ctx.inst.get_byte(0, 3);
    let rd = ctx.inst.get_byte(12, 15);
    let rn = ctx.inst.get_byte(16, 19);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_arg(", ");
    ctx.dis.push_reg_arg(rm, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_str_end_arg("", Some("]"));

    // stole this from the arm LDR instruction i made
    // If register 15 is specified for <Rd>, address[1:0] must be 0b00. If not, the result is UNPREDICTABLE.
    // let value = match bits {
    //     0b00 => bus.read_word(address),
    //     0b01 => bus.read_word(address).rotate_right(8),
    //     0b10 => bus.read_word(address).rotate_right(16),
    //     0b11 => bus.read_word(address).rotate_right(24),
    //     _ => unreachable!(),
    // };
    let rn = ctx.arm.er(rn);
    let bits = rn.get_bits(0, 1); // i have no idea what to call this
    let temp = ctx
        .arm
        .read_word(ctx.bus, ctx.shared, ctx.dma, rn)
        .rotate_right(bits * 8);

    ctx.arm
        .write_word(ctx.bus, ctx.shared, ctx.dma, rn, ctx.arm.er(rm));
    ctx.arm.set_r(rd, temp);

    1 // TODO: this is wrong
}
