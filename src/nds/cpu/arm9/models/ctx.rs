use crate::nds::cpu::{arm9::Arm9, bus::Bus};

use super::Instruction;

pub struct Context<'a, Inst = Instruction> {
    pub inst: Inst,
    pub arm9: &'a mut Arm9,
    pub bus: &'a mut Bus,
}
