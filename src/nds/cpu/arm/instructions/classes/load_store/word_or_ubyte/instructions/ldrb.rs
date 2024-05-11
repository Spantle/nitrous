use crate::nds::cpu::{
    arm::{
        arm::ArmTrait,
        instructions::classes::load_store::word_or_ubyte::LoadStoreInstruction,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

// LDRB
#[inline(always)]
pub fn ldrb(ctx: Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    ctx.arm.set_r(
        ctx.inst.destination_register,
        ctx.bus.read_byte(address) as u32,
    );

    1
}
