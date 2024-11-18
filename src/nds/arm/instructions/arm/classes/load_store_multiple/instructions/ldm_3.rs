use crate::nds::{
    arm::{
        instructions::arm::classes::load_store_multiple::{
            do_writeback, LoadStoreMultipleInstruction,
        },
        models::{Bits, Context, ContextTrait},
        ArmBool, ArmTrait,
    },
    IfElse,
};

// LDM (3)
#[inline(always)]
pub fn ldm_3(
    arm_bool: bool,
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list.get_bit(i as u16) {
            arm.set_r(i, arm.read_word(ctx.bus, ctx.shared, ctx.dma, address));
            address = address.wrapping_add(4);
        }
    }

    arm.set_cpsr(arm.get_spsr());

    let value = arm.read_word(ctx.bus, ctx.shared, ctx.dma, address);
    let cond = arm_bool == ArmBool::ARM9 && arm.cpsr().get_thumb();
    let mask = cond.if_else(0xFFFFFFFE, 0xFFFFFFFC);
    arm.set_r(15, value & mask);

    // address = address.wrapping_add(4);
    // assert end_address = address - 4

    do_writeback(arm_bool, inst_set, ctx);

    1 // TODO: this is not right
}
