use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// STRH (1)
pub fn strh_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STRH");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let immed_5 = ctx.inst.get_word(6, 10);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_word_end_arg(immed_5, Some(", "));
    ctx.dis.push_str_end_arg(" * 2", None);
    ctx.dis.push_str_end_arg("", Some("]"));

    // NOTE: technically it's UNPREDICTABLE if bits 1-0 of address is not 0
    let address = ctx.arm.r()[rn] + (immed_5 * 2);
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
