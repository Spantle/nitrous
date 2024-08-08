use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// STRB (1)
pub fn strb_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STRB");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let immed_5 = ctx.inst.get_word(6, 10);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_word_end_arg(immed_5, Some(", "));
    ctx.dis.push_str_end_arg("", Some("]"));

    let rd = ctx.arm.r()[rd];
    let rn = ctx.arm.r()[rn];
    let address = rn + immed_5;
    ctx.arm
        .write_byte(ctx.bus, ctx.shared, address, rd.get_bits(0, 7) as u8);

    1 // TODO: this is wrong
}
