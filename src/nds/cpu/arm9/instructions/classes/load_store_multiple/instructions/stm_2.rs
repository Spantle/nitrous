use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store_multiple::LoadStoreMultipleInstruction,
        models::{Bits, Context, ContextTrait, ProcessorMode},
    },
    bus::BusTrait,
};

// STM (2)
#[inline(always)]
pub fn stm_2(ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>) -> u32 {
    let (arm9, bus, inst) = (ctx.arm9, ctx.bus, ctx.inst);
    let mut address = inst.start_address;

    let old_mode = arm9.cpsr().get_mode();
    arm9.switch_mode::<false>(ProcessorMode::USR, false);

    for i in 0..=15 {
        if inst.register_list.get_bit(i as u16) {
            bus.write_word(address, arm9.r()[i]);
            address = address.wrapping_add(4);
        }
    }

    arm9.switch_mode::<false>(old_mode, false);

    // assert end_address = address - 4

    1 // TODO: this is not right
}
