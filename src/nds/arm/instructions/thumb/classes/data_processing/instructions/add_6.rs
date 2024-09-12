use crate::nds::arm::{
    instructions::thumb::Instruction,
    models::{Context, ContextTrait, DisassemblyTrait},
    ArmTrait,
};

// ADD (6)
pub fn add_6(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("ADD");

    let rd = ctx.inst.get_byte(8, 10);
    let immed_8 = ctx.inst.get_word(0, 7);
    ctx.dis.push_reg_arg(rd, None);
    ctx.dis.push_str_end_arg("SP", None);
    ctx.dis.push_word_end_arg(immed_8, Some(", "));
    ctx.dis.push_str_end_arg(" * 4", None);

    let sp: u32 = ctx.arm.r()[13];
    let immed_8 = immed_8 << 2;
    let result = sp.wrapping_add(immed_8);

    ctx.arm.set_r(rd, result);

    1 // TODO: this is wrong
}
