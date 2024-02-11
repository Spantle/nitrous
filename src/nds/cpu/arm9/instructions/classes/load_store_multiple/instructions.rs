use crate::nds::cpu::{
    arm9::{
        arm9::Arm9Trait,
        models::{Context, ContextTrait, ProcessorMode},
    },
    bus::BusTrait,
};

use super::LoadStoreMultipleInstruction;

// LDM (1)
#[inline(always)]
pub fn ldm_1(ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>) -> u32 {
    let (arm9, bus, inst) = (ctx.arm9, ctx.bus, ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list >> i & 1 == 1 {
            arm9.r()[i] = bus.read_word(address);
            address += 4;
        }
    }

    if inst.register_list >> 15 & 1 == 1 {
        let value = bus.read_word(address);

        // NOTE: this is for armv5
        arm9.r()[15] = value & 0xFFFFFFFE;
        arm9.cpsr().set_thumb(value & 1 == 1);

        // address += 4;
    }

    // assert end_address = address - 4

    1 // TODO: this is not right
}

// LDM (2)
pub fn ldm_2(ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>) -> u32 {
    let (arm9, bus, inst) = (ctx.arm9, ctx.bus, ctx.inst);
    let mut address = inst.start_address;

    let old_mode = arm9.cpsr().get_mode();
    arm9.switch_mode(ProcessorMode::USR, false);

    for i in 0..=14 {
        if inst.register_list >> i & 1 == 1 {
            arm9.r()[i] = bus.read_word(address);
            address += 4;
        }
    }

    arm9.switch_mode(old_mode, false);

    // assert end_address = address - 4

    1 // TODO: this is not right
}

// LDM (3)
pub fn ldm_3(ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>) -> u32 {
    let (arm9, bus, inst) = (ctx.arm9, ctx.bus, ctx.inst);
    let mut address = inst.start_address;

    for i in 0..=14 {
        if inst.register_list >> i & 1 == 1 {
            arm9.r()[i] = bus.read_word(address);
            address += 4;
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

    // address += 4;
    // assert end_address = address - 4

    1 // TODO: this is not right
}
