use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
        instructions::classes::load_store::word_or_ubyte::LoadStoreInstruction,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

// STR
#[inline(always)]
pub fn str(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    ctx.bus
        .write_word(address, ctx.arm.r()[ctx.inst.destination_register]);

    1
}
