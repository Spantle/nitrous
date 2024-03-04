use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store_multiple::LoadStoreMultipleInstruction,
        models::{Bits, Context, ContextTrait},
    },
    bus::BusTrait,
};

// STM (1)
#[inline(always)]
pub fn stm_1(ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>) -> u32 {
    let (arm9, bus, inst) = (ctx.arm9, ctx.bus, ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=15 {
        if inst.register_list.get_bit(i as u16) {
            bus.write_word(address, arm9.r()[i]);
            address = address.wrapping_add(4);
        }
    }

    // assert end_address = address - 4

    1 // TODO: this is not right
}
