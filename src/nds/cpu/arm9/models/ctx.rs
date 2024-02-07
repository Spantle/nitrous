// thank you for the help with this Leo (@Arduano)

use crate::nds::cpu::{arm9::arm9::Arm9Trait, bus::BusTrait};

use super::disassembly::DisassemblyTrait;

pub struct Context<'a, Inst, Ctx: ContextTrait> {
    pub inst: Inst,
    pub arm9: &'a mut Ctx::Arm9,
    pub bus: &'a mut Ctx::Bus,

    pub dis: &'a mut Ctx::Dis,
}

pub trait ContextTrait {
    type Arm9: Arm9Trait;
    type Bus: BusTrait;

    type Dis: DisassemblyTrait;
}

impl<'a, Inst, Arm9: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait>
    Context<'a, Inst, ContextItems<Arm9, Bus, Dis>>
{
    pub fn new(inst: Inst, arm9: &'a mut Arm9, bus: &'a mut Bus, dis: &'a mut Dis) -> Self {
        Context {
            inst,
            arm9,
            bus,
            dis,
        }
    }
}

// impl<'a, Inst, Ctx: ContextTrait> Context<'a, Inst, Ctx> {
//     pub fn with_inst<NewInst>(&mut self, inst: NewInst) -> Context<'a, NewInst, Ctx> {
//         Context {
//             inst,
//             arm9: &mut self.arm9,
//             bus: &mut self.bus,
//             dis: &mut self.dis,
//         }
//     }
// }

pub struct ContextItems<Arm9: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait> {
    _phantom: std::marker::PhantomData<(Arm9, Bus, Dis)>,
}

impl<Arm9: Arm9Trait, Bus: BusTrait, Dis: DisassemblyTrait> ContextTrait
    for ContextItems<Arm9, Bus, Dis>
{
    type Arm9 = Arm9;
    type Bus = Bus;
    type Dis = Dis;
}
