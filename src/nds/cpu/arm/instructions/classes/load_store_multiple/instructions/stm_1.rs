use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
        instructions::classes::load_store_multiple::{do_writeback, LoadStoreMultipleInstruction},
        models::{Bits, Context, ContextTrait},
    },
    bus::BusTrait,
};

// STM (1)
#[inline(always)]
pub fn stm_1(
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm, bus, inst) = (&mut ctx.arm, &mut ctx.bus, &ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=15 {
        if inst.register_list.get_bit(i as u16) {
            bus.write_word(address, arm.r()[i]);
            address = address.wrapping_add(4);
        }
    }

    // assert end_address = address - 4

    do_writeback(inst_set, ctx);

    1 // TODO: this is not right
}
