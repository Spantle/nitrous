mod addressing_mode;
mod instructions;
mod lookup;

pub use lookup::lookup;

use crate::nds::cpu::arm::{
    arm::ArmTrait,
    models::{Bits, Context, ContextTrait, DisassemblyTrait, Instruction},
};

pub struct LoadStoreMultipleInstruction {
    pub register_list: u16, // bits[15:0] result
    pub start_address: u32, // cheating
    pub end_address: u32,   // cheating

    pub destination: u8,      // cheating (rn)
    pub writeback_value: u32, // cheating
}

impl LoadStoreMultipleInstruction {
    fn new(inst_set: u16, ctx: &mut Context<Instruction, impl ContextTrait>) -> Self {
        let (start_address, end_address, writeback_value) = addressing_mode::parse(inst_set, ctx);
        let register_list = ctx.inst.get_halfword(0, 15);

        ctx.dis.push_str_end_arg("", Some("{"));
        let mut prefix = "";
        for i in 0..=15 {
            if register_list.get_bit(i as u16) {
                ctx.dis.push_reg_end_arg(i, Some(prefix));
                prefix = ",";
            }
        }
        ctx.dis.push_str_end_arg("", Some("}"));

        LoadStoreMultipleInstruction {
            register_list,
            start_address,
            end_address,

            destination: ctx.inst.get_byte(16, 19),
            writeback_value,
        }
    }
}

pub fn do_writeback(inst_set: u16, ctx: Context<LoadStoreMultipleInstruction, impl ContextTrait>) {
    let (arm, inst) = (ctx.arm, ctx.inst);

    // this technically should be in the addressing mode
    let is_writeback = inst_set >> 1 & 1 == 1; // W
    if is_writeback {
        let is_load = inst_set & 1 == 1; // L
        let is_in_register_list = inst.register_list.get_bit(inst.destination as u16);
        let is_first = inst.register_list.trailing_zeros() as u8 == inst.destination;
        // unpredictable behaviour https://discord.com/channels/465585922579103744/667132407262216272/715981285121851503
        if !is_load || !is_in_register_list || is_first {
            arm.r()[inst.destination] = inst.writeback_value;
        }
    }
}
