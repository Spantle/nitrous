use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// LDR (3)
pub fn ldr_3(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("LDR");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7) * 4;
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_end_arg("PC", Some("["));
    ctx.dis.push_word_end_arg(immed_8, Some(", "));
    ctx.dis.push_str_end_arg(" * 4", None);
    ctx.dis.push_str_end_arg("", Some("]"));

    let pc = ctx.arm.r()[15] + 4;
    let address = (pc.get_bits(2, 31) << 2) + immed_8;
    ctx.arm
        .set_r(rd, ctx.arm.read_word(ctx.bus, ctx.shared, address));

    1 // TODO: this is wrong
}
