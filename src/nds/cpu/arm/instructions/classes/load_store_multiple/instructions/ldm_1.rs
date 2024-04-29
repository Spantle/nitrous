use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
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
    let (arm, bus, inst) = (&mut ctx.arm, &ctx.bus, &ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list.get_bit(i as u16) {
            arm.r()[i] = bus.read_word(address);
            address = address.wrapping_add(4);
        }
    }

    if inst.register_list.get_bit(15) {
        let value = bus.read_word(address);

        // NOTE: this is for armv5
        arm.r()[15] = value & 0xFFFFFFFE;
        arm.cpsr().set_thumb(value.get_bit(0));

        // address = address.wrapping_add(4);
    }

    // assert end_address = address - 4

    do_writeback(inst_set, ctx);

    1 // TODO: this is not right
}
