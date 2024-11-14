// thank you for the help with this Leo (@Arduano)

use crate::nds::{
    arm::ArmTrait, bus::BusTrait, dma::DmaTrait, logger::LoggerTrait, shared::Shared,
};

use super::disassembly::DisassemblyTrait;

pub struct Context<'a, Inst, Ctx: ContextTrait> {
    pub inst: Inst,
    pub arm: &'a mut Ctx::Arm,
    pub bus: &'a mut Ctx::Bus,
    pub shared: &'a mut Shared,
    pub dma: &'a mut Ctx::Dma,

    pub dis: &'a mut Ctx::Dis,
    pub logger: &'a mut Ctx::Logger,
}

pub trait ContextTrait {
    type Arm: ArmTrait<Self::Bus, Self::Dma>;
    type Bus: BusTrait<Self::Dma>;
    type Dma: DmaTrait<Self::Bus>;

    type Dis: DisassemblyTrait;
    type Logger: LoggerTrait;
}

impl<
        'a,
        Inst,
        Arm: ArmTrait<Bus, Dma>,
        Bus: BusTrait<Dma>,
        Dma: DmaTrait<Bus>,
        Dis: DisassemblyTrait,
        Logger: LoggerTrait,
    > Context<'a, Inst, ContextItems<Arm, Bus, Dma, Dis, Logger>>
{
    pub fn new(
        inst: Inst,
        arm: &'a mut Arm,
        bus: &'a mut Bus,
        shared: &'a mut Shared,
        dma: &'a mut Dma,
        dis: &'a mut Dis,
        logger: &'a mut Logger,
    ) -> Self {
        Context {
            inst,
            arm,
            bus,
            shared,
            dma,

            dis,
            logger,
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

pub struct ContextItems<
    Arm: ArmTrait<Bus, Dma>,
    Bus: BusTrait<Dma>,
    Dma: DmaTrait<Bus>,
    Dis: DisassemblyTrait,
    Logger: LoggerTrait,
> {
    _phantom: std::marker::PhantomData<(Arm, Bus, Dma, Dis, Logger)>,
}

impl<
        Arm: ArmTrait<Bus, Dma>,
        Bus: BusTrait<Dma>,
        Dma: DmaTrait<Bus>,
        Dis: DisassemblyTrait,
        Logger: LoggerTrait,
    > ContextTrait for ContextItems<Arm, Bus, Dma, Dis, Logger>
{
    type Arm = Arm;
    type Bus = Bus;
    type Dma = Dma;
    type Dis = Dis;
    type Logger = Logger;
}
