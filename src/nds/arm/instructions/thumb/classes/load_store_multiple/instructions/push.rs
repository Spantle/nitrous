use crate::nds::{
    arm::{
        arm::ArmTrait,
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
    },
    Bits,
};

// PUSH
pub fn push(ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("PUSH");

    let register_list = ctx.inst.get_halfword(0, 7);
    let r = ctx.inst.get_bit(8);

    let sp = ctx.arm.r()[13];
    let start_address = sp - 4 * (register_list.count_ones() + r as u32);
    // there's also an end_address but we don't care
    let mut address = start_address;

    for i in 0..=7 {
        if register_list.get_bit(i as u16) {
            ctx.arm
                .write_word(ctx.bus, ctx.shared, address, ctx.arm.r()[i]);
            address = address.wrapping_add(4);
        }
    }
    if r {
        ctx.arm
            .write_word(ctx.bus, ctx.shared, address, ctx.arm.r()[14]);
        // address = address.wrapping_add(4);
    }

    ctx.arm.set_r(13, start_address);
}
