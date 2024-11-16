use crate::nds::{
    arm::{
        instructions::thumb::Instruction,
        models::{Context, ContextTrait, DisassemblyTrait},
        ArmBool, ArmTrait,
    },
    Bits,
};

// POP
pub fn pop(arm_bool: bool, ctx: &mut Context<Instruction, impl ContextTrait>) {
    ctx.dis.set_inst("POP");

    let register_list = ctx.inst.get_halfword(0, 7);
    let r = ctx.inst.get_bit(8);

    let sp = ctx.arm.r()[13];
    let start_address = sp;
    // there's also an end_address but we don't care
    let mut address = start_address;

    for i in 0..=7 {
        if register_list.get_bit(i as u16) {
            ctx.arm
                .set_r(i, ctx.arm.read_word(ctx.bus, ctx.shared, ctx.dma, address));
            address = address.wrapping_add(4);
        }
    }
    if r {
        let value = ctx.arm.read_word(ctx.bus, ctx.shared, ctx.dma, address);
        ctx.arm.set_r(15, value & 0xFFFFFFFE);
        if arm_bool == ArmBool::ARM9 {
            ctx.arm.cpsr_mut().set_thumb(value.get_bit(0));
        }
        address = address.wrapping_add(4);
    }

    ctx.arm.set_r(13, address);
}
