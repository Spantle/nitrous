use crate::nds::arm::{
    arm::ArmTrait,
    instructions::arm::classes::load_store_multiple::{do_writeback, LoadStoreMultipleInstruction},
    models::{Bits, Context, ContextTrait},
};

// LDM (3)
#[inline(always)]
pub fn ldm_3(
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list.get_bit(i as u16) {
            arm.set_r(i, arm.read_word(ctx.bus, ctx.shared, address));
            address = address.wrapping_add(4);
        }
    }

    arm.set_cpsr(arm.get_spsr());

    let value = arm.read_word(ctx.bus, ctx.shared, address);
    // NOTE: this is for armv5
    if arm.cpsr().get_thumb() {
        arm.set_r(15, value & 0xFFFFFFFE);
    } else {
        arm.set_r(15, value & 0xFFFFFFFC);
    }

    // address = address.wrapping_add(4);
    // assert end_address = address - 4

    do_writeback(inst_set, ctx);

    1 // TODO: this is not right
}
