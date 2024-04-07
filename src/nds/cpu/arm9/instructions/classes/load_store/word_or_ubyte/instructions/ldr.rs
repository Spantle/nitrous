use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        instructions::classes::load_store::word_or_ubyte::LoadStoreInstruction,
        models::{Bits, Context, ContextTrait},
    },
    bus::BusTrait,
};

// LDR
#[inline(always)]
pub fn ldr(ctx: Context<LoadStoreInstruction, impl ContextTrait>, address: u32) -> u32 {
    let (arm9, bus, inst) = (ctx.arm9, ctx.bus, ctx.inst);
    let bits = address.get_bits(0, 1); // i have no idea what to call this
    let mut cycles = 1 + (bits != 0) as u32;

    // If register 15 is specified for <Rd>, address[1:0] must be 0b00. If not, the result is UNPREDICTABLE.
    // let value = match bits {
    //     0b00 => bus.read_word(address),
    //     0b01 => bus.read_word(address).rotate_right(8),
    //     0b10 => bus.read_word(address).rotate_right(16),
    //     0b11 => bus.read_word(address).rotate_right(24),
    //     _ => unreachable!(),
    // };
    let value = bus.read_word(address).rotate_right(bits * 8);

    if inst.destination_register == 15 {
        // note: this is for armv5
        arm9.r()[15] = value & 0xFFFFFFFE;
        arm9.cpsr().set_thumb(value.get_bit(0));
        cycles += 4;
    } else {
        arm9.r()[inst.destination_register] = value;
    }

    cycles
}
