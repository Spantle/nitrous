use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmTrait,
    },
    Bits,
};

// STMIA
pub fn stmia(ctx: &mut Context<Instruction, impl ContextTrait>) -> u32 {
    ctx.dis.set_inst("STMIA");

    let rn = ctx.inst.get_byte(8, 10);
    let register_list = ctx.inst.get_halfword(0, 7);
    ctx.dis.push_reg_arg(rn, Some("!"));

    let start_address = ctx.arm.r()[rn];
    // there's also an end_address but we don't care
    let mut address = start_address;

    for i in 0..=7 {
        if register_list.get_bit(i as u16) {
            ctx.arm
                .write_word(ctx.bus, ctx.shared, ctx.dma, address, ctx.arm.r()[i]);
            address = address.wrapping_add(4);
        }
    }

    ctx.arm
        .set_r(rn, ctx.arm.r()[rn] + (register_list.count_ones() * 4));

    1 // TODO: this is wrong
}
