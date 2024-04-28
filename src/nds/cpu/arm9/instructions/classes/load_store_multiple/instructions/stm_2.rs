use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store_multiple::{do_writeback, LoadStoreMultipleInstruction},
        models::{Bits, Context, ContextTrait, ProcessorMode},
    },
    bus::BusTrait,
};

// STM (2)
#[inline(always)]
pub fn stm_2(
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm9, bus, inst) = (&mut ctx.arm9, &mut ctx.bus, &ctx.inst);
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

    // this technically should be in the addressing mode
    let is_writeback = inst_set >> 1 & 1 == 1; // W
    if is_writeback {
        arm9.r()[inst.destination] = inst.writeback_value;
    }

    do_writeback(inst_set, ctx);

    1 // TODO: this is not right
}
