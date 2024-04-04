use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store::word_or_ubyte::LoadStoreInstruction,
        models::{Context, ContextTrait},
    },
    bus::BusTrait,
};

// LDRB
#[inline(always)]
pub fn ldrb(ctx: Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    ctx.arm9.r()[ctx.inst.destination_register] = ctx.bus.read_byte(address) as u32;

    1
}
