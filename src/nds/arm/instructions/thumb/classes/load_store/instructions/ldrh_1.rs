use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// LDRH (1)
pub fn ldrh_1(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LDRH");

    let rd = ctx.inst.get_byte(0, 2);
    let rn = ctx.inst.get_byte(3, 5);
    let immed_5 = ctx.inst.get_word(6, 10);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_reg_end_arg(rn, Some("["));
    ctx.dis.push_word_end_arg(immed_5, Some(", "));
    ctx.dis.push_str_end_arg(" * 2", None);
    ctx.dis.push_str_end_arg("", Some("]"));

    let address = ctx.arm.r()[rn] + (immed_5 * 2);
    // NOTE: technically it's UNPREDICTABLE if bit 0 of address is not 0
    ctx.arm.set_r(
        rd,
        ctx.arm.read_halfword(ctx.bus, ctx.shared, ctx.dma, address) as u32,
    );

    1 // TODO: this is wrong
}
