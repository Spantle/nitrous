use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store_multiple::{do_writeback, LoadStoreMultipleInstruction},
        models::{Bits, Context, ContextTrait},
    },
    bus::BusTrait,
};

// LDM (3)
#[inline(always)]
pub fn ldm_3(
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm9, bus, inst) = (&mut ctx.arm9, &mut ctx.bus, &mut ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list.get_bit(i as u16) {
            arm9.r()[i] = bus.read_word(address);
            address = address.wrapping_add(4);
        }
    }

    arm9.set_cpsr(arm9.get_spsr());

    let value = bus.read_word(address);
    // NOTE: this is for armv5
    if arm9.cpsr().get_thumb() {
        arm9.r()[15] = value & 0xFFFFFFFE;
    } else {
        arm9.r()[15] = value & 0xFFFFFFFC;
    }

    // address = address.wrapping_add(4);
    // assert end_address = address - 4

    do_writeback(inst_set, ctx);

    1 // TODO: this is not right
}
