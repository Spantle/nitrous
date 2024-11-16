use crate::nds::{bus::BusTrait, dma::Dma, logger, shared::Shared};

use super::{models::PowerDownMode, Arm, ArmKind, ArmTrait};

pub trait ArmInternalRW<Bus: BusTrait> {
    fn read_bulk(
        &self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        addr: u32,
        len: u32,
    ) -> Vec<u8>;
    fn write_bulk(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        addr: u32,
        data: Vec<u8>,
    );
    fn read_slice<const T: usize>(
        &self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        addr: u32,
    ) -> [u8; T];
    fn write_slice<const T: usize>(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        addr: u32,
        value: [u8; T],
    );
}

impl<Bus: BusTrait> ArmInternalRW<Bus> for Arm<Bus> {
    fn read_bulk(
        &self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        addr: u32,
        len: u32,
    ) -> Vec<u8> {
        let mut bytes = vec![];
        for i in 0..len {
            bytes.push(self.read_byte(bus, shared, dma, addr + i));
        }
        bytes
    }

    fn write_bulk(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        addr: u32,
        data: Vec<u8>,
    ) {
        (0..data.len()).for_each(|i| {
            self.write_byte(bus, shared, dma, addr + i as u32, data[i]);
        });
    }

    #[inline(always)]
    fn read_slice<const T: usize>(
        &self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        orig_addr: u32,
    ) -> [u8; T] {
        let addr = orig_addr as usize / T * T;
        let mut bytes = [0; T];

        let (data_tcm_base, data_tcm_size, inst_tcm_base, inst_tcm_size) = (
            self.cp15.data_tcm_base as usize,
            self.cp15.data_tcm_size as usize,
            self.cp15.inst_tcm_base as usize,
            self.cp15.inst_tcm_size as usize,
        );
        let (data_tcm_end, inst_tcm_end) =
            (data_tcm_base + data_tcm_size, inst_tcm_base + inst_tcm_size);
        match Bus::KIND {
            ArmKind::Arm9 => {
                if !self.cp15.control_register.get_instruction_tcm_load_mode()
                    && addr >= inst_tcm_base
                    && addr < inst_tcm_end
                {
                    let addr = (addr - inst_tcm_base) % self.cp15.inst_tcm.len();
                    bytes.copy_from_slice(&self.cp15.inst_tcm[addr..addr + T]);
                    return bytes;
                }
                if !self.cp15.control_register.get_data_tcm_load_mode()
                    && addr >= data_tcm_base
                    && addr < data_tcm_end
                {
                    let addr = (addr - data_tcm_base) % self.cp15.data_tcm.len();
                    bytes.copy_from_slice(&self.cp15.data_tcm[addr..addr + T]);
                    return bytes;
                }

                bus.read_slice::<T>(shared, &mut Some(dma), orig_addr)
            }
            ArmKind::Arm7 => bus.read_slice::<T>(shared, &mut Some(dma), orig_addr),
        }
    }

    #[inline(always)]
    fn write_slice<const T: usize>(
        &mut self,
        bus: &mut Bus,
        shared: &mut Shared,
        dma: &mut Dma,
        orig_addr: u32,
        value: [u8; T],
    ) {
        let addr = orig_addr as usize / T * T;

        match Bus::KIND {
            ArmKind::Arm9 => {
                let (data_tcm_base, data_tcm_size, inst_tcm_base, inst_tcm_size) = (
                    self.cp15.data_tcm_base as usize,
                    self.cp15.data_tcm_size as usize,
                    self.cp15.inst_tcm_base as usize,
                    self.cp15.inst_tcm_size as usize,
                );
                let (data_tcm_end, inst_tcm_end) =
                    (data_tcm_base + data_tcm_size, inst_tcm_base + inst_tcm_size);

                if addr >= inst_tcm_base && addr < inst_tcm_end {
                    let addr = (addr - inst_tcm_base) % self.cp15.inst_tcm.len();
                    self.cp15.inst_tcm[addr..addr + T].copy_from_slice(&value);
                    return;
                }
                if addr >= data_tcm_base && addr < data_tcm_end {
                    let addr = (addr - data_tcm_base) % self.cp15.data_tcm.len();
                    self.cp15.data_tcm[addr..addr + T].copy_from_slice(&value);
                    return;
                }

                bus.write_slice::<T>(shared, &mut Some(dma), orig_addr, value)
            }
            ArmKind::Arm7 => match addr {
                0x04000301 => {
                    self.haltcnt.set(value[0]);

                    // cannot be bothered doing this inside of a set function, i dont think it would be possible
                    let log_source = logger::LogSource::Arm7(self.r[15]);
                    let power_down_mode = self.haltcnt.get_power_down_mode();
                    match power_down_mode {
                        PowerDownMode::ENTER_GBA_MODE => {
                            logger::error(
                                log_source,
                                "Processor tried to enter GBA mode! Not implemented!",
                            );
                        }
                        PowerDownMode::HALT => self.halt(),
                        PowerDownMode::SLEEP => {
                            logger::error(
                                log_source,
                                "Processor tried to enter sleep mode! Not implemented!",
                            );
                        }
                        _ => {}
                    }
                }
                _ => bus.write_slice::<T>(shared, &mut Some(dma), orig_addr, value),
            },
        };
    }
}
