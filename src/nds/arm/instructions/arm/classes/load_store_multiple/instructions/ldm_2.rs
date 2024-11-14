use crate::nds::arm::{
    instructions::arm::classes::load_store_multiple::{do_writeback, LoadStoreMultipleInstruction},
    models::{Bits, Context, ContextTrait, ProcessorMode},
    ArmTrait,
};

// LDM (2)
#[inline(always)]
pub fn ldm_2(
    arm_bool: bool,
    inst_set: u16,
    mut ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>,
) -> u32 {
    let (arm, inst) = (&mut ctx.arm, &ctx.inst);
    let mut address = inst.start_address;

    let old_mode = arm.cpsr().get_mode();
    arm.switch_mode::<false>(ProcessorMode::USR, false);

    for i in 0..=14 {
        if inst.register_list.get_bit(i as u16) {
            arm.set_r(i, arm.read_word(ctx.bus, ctx.shared, ctx.dma,address));
            address = address.wrapping_add(4);
        }
    }

    arm.switch_mode::<false>(old_mode, false);

    // assert end_address = address - 4

    do_writeback(arm_bool, inst_set, ctx);

    1 // TODO: this is not right
}
