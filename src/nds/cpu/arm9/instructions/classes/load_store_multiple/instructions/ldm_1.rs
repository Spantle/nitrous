use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store_multiple::{do_writeback, LoadStoreMultipleInstruction},
        models::{Bits, Context, ContextTrait},
    },
    bus::BusTrait,
};

// LDM (1)
#[inline(always)]
pub fn ldm_1(
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm9, bus, inst) = (&mut ctx.arm9, &ctx.bus, &ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list.get_bit(i as u16) {
            arm9.r()[i] = bus.read_word(address);
            address = address.wrapping_add(4);
        }
    }

    if inst.register_list.get_bit(15) {
        let value = bus.read_word(address);

        // NOTE: this is for armv5
        arm9.r()[15] = value & 0xFFFFFFFE;
        arm9.cpsr().set_thumb(value.get_bit(0));

        // address = address.wrapping_add(4);
    }

    // assert end_address = address - 4

    do_writeback(inst_set, ctx);

    1 // TODO: this is not right
}
