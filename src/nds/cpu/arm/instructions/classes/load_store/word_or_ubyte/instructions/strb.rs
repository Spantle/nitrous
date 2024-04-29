use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
        instructions::classes::load_store::word_or_ubyte::LoadStoreInstruction,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

// STRB
#[inline(always)]
pub fn strb(ctx: &mut Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    ctx.bus
        .write_byte(address, ctx.arm.eru(ctx.inst.destination_register) as u8);

    1
}
