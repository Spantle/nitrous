use crate::nds::{
    arm::ArmKind,
    div::DividerUnit,
    dma::Dma,
    interrupts::Interrupts,
    logger::{self, format_debug, Logger, LoggerTrait},
    shared::Shared,
    sqrt::SquareRootUnit,
    timers::Timers,
    Bits, Bytes,
};

use super::BusTrait;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Bus9 {
    logger: Logger,

    #[serde(skip)]
    pub bios: Vec<u8>,
    pub interrupts: Interrupts,

    pub timers: Timers,
    pub div: DividerUnit,
    pub sqrt: SquareRootUnit,
}

impl Default for Bus9 {
    fn default() -> Self {
        Self {
            logger: Logger(logger::LogSource::Bus9),

            bios: Vec::new(),
            interrupts: Interrupts::default(),

            timers: Timers::default(),
            div: DividerUnit::default(),
            sqrt: SquareRootUnit::default(),
        }
    }
}

impl BusTrait for Bus9 {
    const KIND: ArmKind = ArmKind::Arm9;

    fn reset(&mut self) {
        self.interrupts = Interrupts::default();
        self.timers = Timers::default();
        self.div = DividerUnit::default();
    }

    fn load_state(&mut self, bus: Self) {
        self.interrupts = bus.interrupts;
        self.timers = bus.timers;
        self.div = bus.div;
    }

    fn load_bios(&mut self, bios: Vec<u8>) {
        self.logger.log_info("Successfully loaded BIOS");
        self.bios = bios;
    }

    fn load_bios_from_path(&mut self, path: &str) {
        let file = std::fs::read(path);
        match file {
            Ok(bios) => self.load_bios(bios),
            Err(e) => self.logger.log_error(format!("Failed to load BIOS: {}", e)),
        };
    }

    fn load_firmware(&mut self, _firmware: Vec<u8>) {
        self.logger.log_error("Tried to load firmware on ARM9");
    }

    fn load_firmware_from_path(&mut self, path: &str) {
        self.logger.log_error(format!(
            "Tried to load firmware on ARM9 from path: {}",
            path
        ));
    }

    fn is_requesting_interrupt(&self) -> bool {
        self.interrupts.is_requesting_interrupt()
    }
    fn get_interrupts(&mut self) -> &mut Interrupts {
        &mut self.interrupts
    }

    fn read_byte(&self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32) -> u8 {
        self.read_slice::<1>(shared, dma, addr)[0]
    }
    fn read_halfword(&self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32) -> u16 {
        let bytes = self.read_slice::<2>(shared, dma, addr);
        u16::from_le_bytes(bytes)
    }
    fn read_word(&self, shared: &mut Shared, dma: &mut Option<&mut Dma>, addr: u32) -> u32 {
        let bytes = self.read_slice::<4>(shared, dma, addr);
        u32::from_le_bytes(bytes)
    }

    fn write_byte(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: u8,
    ) {
        self.write_slice::<1>(shared, dma, addr, [value]);
    }
    fn write_halfword(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: u16,
    ) {
        self.write_slice::<2>(shared, dma, addr, value.to_le_bytes());
    }
    fn write_word(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: u32,
    ) {
        self.write_slice::<4>(shared, dma, addr, value.to_le_bytes());
    }

    #[inline(always)]
    fn read_slice<const T: usize>(
        &self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
    ) -> [u8; T] {
        let addr = addr as usize / T * T;
        let mut bytes = [0; T];
        match addr {
            0x00000000..=0x00000001 => bytes, // not real

            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 0x400000;
                bytes.copy_from_slice(&shared.psram[addr..addr + T]);
                bytes
            }

            0x03000000..=0x03FFFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                bytes.copy_from_slice(&shared.wram[addr..addr + T]);
                bytes
            }

            0x04000000..=0x04000003 => shared.gpus.a.dispcnt.value().to_bytes::<T>(),
            0x04001000..=0x04001003 => shared.gpus.b.dispcnt.value().to_bytes::<T>(),

            0x04000004..=0x04000005 => shared.gpus.dispstat.value().to_bytes::<T>(),
            0x04000006..=0x04000007 => shared.gpus.vcount.to_bytes::<T>(),

            0x04000020..=0x0400004F => {
                self.logger.log_warn_once(format_debug!(
                    "GPU feature not implemented (R{} {:#010X})",
                    T,
                    addr
                ));
                bytes
            }
            0x04001020..=0x0400104F => {
                self.logger.log_warn_once(format_debug!(
                    "GPU feature not implemented (R{} {:#010X})",
                    T,
                    addr
                ));
                bytes
            }

            0x04000008..=0x04000009 => shared.gpus.a.bgxcnt[0].value().to_bytes::<T>(),
            0x0400000A..=0x0400000B => shared.gpus.a.bgxcnt[1].value().to_bytes::<T>(),
            0x0400000C..=0x0400000D => shared.gpus.a.bgxcnt[2].value().to_bytes::<T>(),
            0x0400000E..=0x0400000F => shared.gpus.a.bgxcnt[3].value().to_bytes::<T>(),

            0x04001008..=0x04001009 => shared.gpus.b.bgxcnt[0].value().to_bytes::<T>(),
            0x0400100A..=0x0400100B => shared.gpus.b.bgxcnt[1].value().to_bytes::<T>(),
            0x0400100C..=0x0400100D => shared.gpus.b.bgxcnt[2].value().to_bytes::<T>(),
            0x0400100E..=0x0400100F => shared.gpus.b.bgxcnt[3].value().to_bytes::<T>(),

            0x04000050..=0x04000051 => shared.gpus.a.bldcnt.value().to_bytes::<T>(),
            0x04000052 => shared.gpus.a.bldalpha[0].value().to_bytes::<T>(),
            0x04000053 => shared.gpus.a.bldalpha[1].value().to_bytes::<T>(),

            0x04001050..=0x04001051 => shared.gpus.b.bldcnt.value().to_bytes::<T>(),
            0x04001052 => shared.gpus.b.bldalpha[0].value().to_bytes::<T>(),
            0x04001053 => shared.gpus.b.bldalpha[1].value().to_bytes::<T>(),

            0x04000060..=0x04000061 => {
                self.logger.log_warn_once(format_debug!(
                    "GPU3D not implemented (R{} {:#010X})",
                    T,
                    addr
                ));
                bytes
            }

            0x04000100..=0x04000101 => self.timers.get(0).get_counter().to_bytes::<T>(),
            0x04000102..=0x04000103 => self.timers.get(0).get_control().to_bytes::<T>(),
            0x04000104..=0x04000105 => self.timers.get(1).get_counter().to_bytes::<T>(),
            0x04000106..=0x04000107 => self.timers.get(1).get_control().to_bytes::<T>(),
            0x04000108..=0x04000109 => self.timers.get(2).get_counter().to_bytes::<T>(),
            0x0400010A..=0x0400010B => self.timers.get(2).get_control().to_bytes::<T>(),
            0x0400010C..=0x0400010D => self.timers.get(3).get_counter().to_bytes::<T>(),
            0x0400010E..=0x0400010F => self.timers.get(3).get_control().to_bytes::<T>(),

            0x04000130..=0x04000131 => shared.keyinput.value().to_bytes::<T>(),
            0x04000132..=0x04000133 => {
                self.logger.log_warn_once(format_debug!(
                    "KEYCNT not implemented (R{} {:#010X})",
                    T,
                    addr
                ));
                bytes
            }
            0x04000136..=0x04000137 => shared.extkeyin.value().to_bytes::<T>(),

            0x04000180..=0x04000183 => shared.ipcsync.value::<true>().to_bytes::<T>(),
            0x04000184..=0x04000185 => shared.ipcfifo.get_cnt::<true>().to_bytes::<T>(),

            0x040001A0..=0x040001A1 => shared.cart.auxspicnt.value().to_bytes::<T>(),
            0x040001A4..=0x040001A7 => shared.cart.romctrl.value().to_bytes::<T>(),

            0x04000204..=0x04000205 => shared.cart.exmemcnt.0.to_bytes::<T>(),

            0x04000208..=0x0400020B => self.interrupts.me.value().to_bytes::<T>(),
            0x04000210..=0x04000213 => self.interrupts.e.value().to_bytes::<T>(),
            0x04000214..=0x04000217 => self.interrupts.f.value().to_bytes::<T>(),

            0x04000240..=0x04000249 => {
                let addr = addr - 0x04000240;
                let mut result = [0; T];
                #[allow(clippy::needless_range_loop)] // i would like to use the constant
                for i in 0..T {
                    result[i] = match addr + i {
                        0 => shared.gpus.vram_banks.a.read_vramcnt(),
                        1 => shared.gpus.vram_banks.b.read_vramcnt(),
                        2 => shared.gpus.vram_banks.c.read_vramcnt(),
                        3 => shared.gpus.vram_banks.d.read_vramcnt(),
                        4 => shared.gpus.vram_banks.e.read_vramcnt(),
                        5 => shared.gpus.vram_banks.f.read_vramcnt(),
                        6 => shared.gpus.vram_banks.g.read_vramcnt(),
                        7 => shared.gpus.wramcnt,
                        8 => shared.gpus.vram_banks.h.read_vramcnt(),
                        9 => shared.gpus.vram_banks.i.read_vramcnt(),
                        _ => unreachable!(),
                    };
                }

                result
            }

            0x04000280..=0x04000283 => self.div.control.value().to_bytes::<T>(),
            0x04000290..=0x04000293 => self.div.numerator_lo.to_bytes::<T>(),
            0x04000294..=0x04000297 => self.div.numerator_hi.to_bytes::<T>(),
            0x04000298..=0x0400029B => self.div.denominator_lo.to_bytes::<T>(),
            0x0400029C..=0x0400029F => self.div.denominator_hi.to_bytes::<T>(),
            0x040002A0..=0x040002A3 => self.div.result_lo.to_bytes::<T>(),
            0x040002A4..=0x040002A7 => self.div.result_hi.to_bytes::<T>(),
            0x040002A8..=0x040002AB => self.div.remainder_lo.to_bytes::<T>(),
            0x040002AC..=0x040002AF => self.div.remainder_hi.to_bytes::<T>(),

            0x040002B0 => self.sqrt.control.value().to_bytes::<T>(),
            0x040002B4 => self.sqrt.result.to_bytes::<T>(),
            0x040002B8 => self.sqrt.param_lo.to_bytes::<T>(),
            0x040002BC => self.sqrt.param_hi.to_bytes::<T>(),

            0x04000300 => shared.postflg.0.to_bytes::<T>(),
            0x04000304..=0x04000307 => shared.powcnt1.value().to_bytes::<T>(),

            0x04000320..=0x040006A4 => {
                self.logger.log_warn_once(format_debug!(
                    "GPU3D not implemented (R{} {:#010X})",
                    T,
                    addr
                ));
                bytes
            }

            0x04004000..=0x04004001 => bytes, // DSi Stuff, return nothing
            0x04004008..=0x0400400B => bytes, // DSi Stuff, return nothing

            0x04100000..=0x04100003 => shared.ipcfifo.receive::<true>().to_bytes::<T>(),
            0x04100010..=0x04100013 => shared.cart.read_bus().to_bytes::<T>(),

            0x05000000..=0x05FFFFFF => {
                let addr = (addr - 0x05000000) % 0x800;
                if addr < 0x400 {
                    bytes.copy_from_slice(&shared.gpus.a.palette[addr..addr + T]);
                    bytes
                } else {
                    let addr = addr - 0x400;
                    bytes.copy_from_slice(&shared.gpus.b.palette[addr..addr + T]);
                    bytes
                }
            }

            0x08000000..=0x0AFFFFFF => bytes, // gba slot, return nothing... for now?

            0xFFFF0000..=0xFFFF7FFF => {
                let addr = addr - 0xFFFF0000;
                bytes.copy_from_slice(&self.bios[addr..addr + T]);
                bytes
            }

            _ => {
                if let Some(dma) = dma {
                    if let Some(bytes) = dma.read_slice::<T>(addr) {
                        return bytes;
                    }
                }

                if let Some(bytes) = shared.gpus.vram_banks.read_slice::<T>(addr) {
                    return bytes;
                }

                self.logger.log_error_once(format!(
                    "Invalid read {} byte(s) at address {:#010X}",
                    T, addr
                ));
                bytes
            }
        }
    }

    #[inline(always)]
    fn write_slice<const T: usize>(
        &mut self,
        shared: &mut Shared,
        dma: &mut Option<&mut Dma>,
        addr: u32,
        value: [u8; T],
    ) {
        let addr = addr as usize / T * T;
        match addr {
            0x00000000..=0x00000001 => {} // not real

            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 0x400000;
                shared.psram[addr..addr + T].copy_from_slice(&value);
            }

            0x03000000..=0x03FFFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                shared.wram[addr..addr + T].copy_from_slice(&value);
            }

            0x04000000..=0x04000003 => shared.gpus.a.dispcnt = value.into_word().into(),
            0x04001000..=0x04001003 => shared.gpus.b.dispcnt = value.into_word().into(),
            0x04000004..=0x04000005 => shared.gpus.dispstat = (value.into_word() as u16).into(),

            0x04000008..=0x0400000F => {
                for i in 0..T {
                    let value = value[i] as u16;
                    match addr + i {
                        0x04000008 => shared.gpus.a.bgxcnt[0].0.set_bits(0, 7, value),
                        0x04000009 => shared.gpus.a.bgxcnt[0].0.set_bits(8, 15, value),
                        0x0400000A => shared.gpus.a.bgxcnt[1].0.set_bits(0, 7, value),
                        0x0400000B => shared.gpus.a.bgxcnt[1].0.set_bits(8, 15, value),
                        0x0400000C => shared.gpus.a.bgxcnt[2].0.set_bits(0, 7, value),
                        0x0400000D => shared.gpus.a.bgxcnt[2].0.set_bits(8, 15, value),
                        0x0400000E => shared.gpus.a.bgxcnt[3].0.set_bits(0, 7, value),
                        0x0400000F => shared.gpus.a.bgxcnt[3].0.set_bits(8, 15, value),
                        _ => unreachable!(),
                    }
                }
            }

            0x04001008..=0x0400100F => {
                for i in 0..T {
                    let value = value[i] as u16;
                    match addr + i {
                        0x04001008 => shared.gpus.b.bgxcnt[0].0.set_bits(0, 7, value),
                        0x04001009 => shared.gpus.b.bgxcnt[0].0.set_bits(8, 15, value),
                        0x0400100A => shared.gpus.b.bgxcnt[1].0.set_bits(0, 7, value),
                        0x0400100B => shared.gpus.b.bgxcnt[1].0.set_bits(8, 15, value),
                        0x0400100C => shared.gpus.b.bgxcnt[2].0.set_bits(0, 7, value),
                        0x0400100D => shared.gpus.b.bgxcnt[2].0.set_bits(8, 15, value),
                        0x0400100E => shared.gpus.b.bgxcnt[3].0.set_bits(0, 7, value),
                        0x0400100F => shared.gpus.b.bgxcnt[3].0.set_bits(8, 15, value),
                        _ => unreachable!(),
                    }
                }
            }
            0x04000010..=0x04000013 => {
                shared.gpus.a.bgofs[0].set_part::<T>(addr as u32 - 0x04000010, value.into_word())
            }
            0x04000014..=0x04000017 => {
                shared.gpus.a.bgofs[1].set_part::<T>(addr as u32 - 0x04000014, value.into_word());
            }
            0x04000018..=0x0400001B => {
                shared.gpus.a.bgofs[2].set_part::<T>(addr as u32 - 0x04000018, value.into_word());
            }
            0x0400001C..=0x0400001F => {
                shared.gpus.a.bgofs[3].set_part::<T>(addr as u32 - 0x0400001C, value.into_word());
            }

            0x04001010..=0x04001013 => {
                shared.gpus.b.bgofs[0].set_part::<T>(addr as u32 - 0x04001010, value.into_word());
            }
            0x04001014..=0x04001017 => {
                shared.gpus.b.bgofs[1].set_part::<T>(addr as u32 - 0x04001014, value.into_word());
            }
            0x04001018..=0x0400101B => {
                shared.gpus.b.bgofs[2].set_part::<T>(addr as u32 - 0x04001018, value.into_word());
            }
            0x0400101C..=0x0400101F => {
                shared.gpus.b.bgofs[3].set_part::<T>(addr as u32 - 0x0400101C, value.into_word());
            }

            0x04000020..=0x0400004F => self.logger.log_warn_once(format_debug!(
                "GPU feature not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04001020..=0x0400104F => self.logger.log_warn_once(format_debug!(
                "GPU feature not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04000050..=0x04000055 => {
                for i in 0..T {
                    let value = value[i];
                    match addr + i {
                        0x04000050 => shared.gpus.a.bldcnt.0.set_bits(0, 7, value as u16),
                        0x04000051 => shared.gpus.a.bldcnt.0.set_bits(8, 15, value as u16),
                        0x04000052 => shared.gpus.a.bldalpha[0] = value.into(),
                        0x04000053 => shared.gpus.a.bldalpha[1] = value.into(),
                        0x04000054 => shared.gpus.a.bldy[0] = value,
                        0x04000055 => shared.gpus.a.bldy[1] = value,
                        _ => {}
                    }
                }
            }
            0x04001050..=0x04001055 => {
                for i in 0..T {
                    let value = value[i];
                    match addr + i {
                        0x04001050 => shared.gpus.b.bldcnt.0.set_bits(0, 7, value as u16),
                        0x04001051 => shared.gpus.b.bldcnt.0.set_bits(8, 15, value as u16),
                        0x04001052 => shared.gpus.b.bldalpha[0] = value.into(),
                        0x04001053 => shared.gpus.b.bldalpha[1] = value.into(),
                        0x04001054 => shared.gpus.b.bldy[0] = value,
                        0x04001055 => shared.gpus.b.bldy[1] = value,
                        _ => {}
                    }
                }
            }

            0x0400005C..=0x0400005F => {} // not real

            0x04000060..=0x04000061 => self.logger.log_warn_once(format_debug!(
                "GPU3D not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04000064..=0x04000067 => self.logger.log_warn_once(format_debug!(
                "DISPCAPCNT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04000068..=0x0400006B => self.logger.log_warn_once(format_debug!(
                "DISP_MMEM_FIFO not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x0400006C..=0x0400006D => self.logger.log_warn_once(format_debug!(
                "MASTER_BRIGHT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x0400106C..=0x0400106D => self.logger.log_warn_once(format_debug!(
                "MASTER_BRIGHT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04000100..=0x04000101 => self.timers.get_mut(0).set_l(value.into_halfword()),
            0x04000102..=0x04000103 => self.timers.get_mut(0).set_h(value.into_halfword()),
            0x04000104..=0x04000105 => self.timers.get_mut(1).set_l(value.into_halfword()),
            0x04000106..=0x04000107 => self.timers.get_mut(1).set_h(value.into_halfword()),
            0x04000108..=0x04000109 => self.timers.get_mut(2).set_l(value.into_halfword()),
            0x0400010A..=0x0400010B => self.timers.get_mut(2).set_h(value.into_halfword()),
            0x0400010C..=0x0400010D => self.timers.get_mut(3).set_l(value.into_halfword()),
            0x0400010E..=0x0400010F => self.timers.get_mut(3).set_h(value.into_halfword()),

            0x04000132..=0x04000133 => self.logger.log_warn_once(format_debug!(
                "KEYCNT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04000180..=0x04000183 => shared.ipcsync.set::<true, T>(addr - 0x04000180, value),
            0x04000184..=0x04000187 => shared
                .ipcfifo
                .set_cnt::<true>(&mut self.interrupts, value.into_word()),
            0x04000188..=0x0400018B => shared.ipcfifo.send::<true>(value.into_word()),

            0x040001A1 => shared.cart.auxspicnt.set_hi(value.into_halfword()),
            0x040001A0..=0x040001A1 => shared.cart.auxspicnt.set(value.into_halfword()),
            0x040001A4..=0x040001A7 => shared.cart.romctrl.set(value.into_word()),
            0x040001A8..=0x040001AF => {
                shared
                    .cart
                    .command
                    .update(addr - 0x040001A8, T, value.into_word())
            }

            0x04000204..=0x04000205 => shared.cart.exmemcnt.0 = value.into_halfword(),

            0x04000208..=0x0400020B => self.interrupts.me = value.into_word().into(),
            0x04000210..=0x04000213 => self.interrupts.e = value.into_word().into(),
            0x04000214..=0x04000217 => self.interrupts.f.write_and_ack(value.into_word()),

            0x04000240..=0x04000249 => {
                // this sucks so much
                let addr = addr - 0x04000240;
                #[allow(clippy::needless_range_loop)] // i would like to use the constant
                for i in 0..T {
                    match addr + i {
                        0 => shared.gpus.vram_banks.a.write_vramcnt(value[i]),
                        1 => shared.gpus.vram_banks.b.write_vramcnt(value[i]),
                        2 => shared.gpus.vram_banks.c.write_vramcnt(value[i]),
                        3 => shared.gpus.vram_banks.d.write_vramcnt(value[i]),
                        4 => shared.gpus.vram_banks.e.write_vramcnt(value[i]),
                        5 => shared.gpus.vram_banks.f.write_vramcnt(value[i]),
                        6 => shared.gpus.vram_banks.g.write_vramcnt(value[i]),
                        7 => shared.gpus.wramcnt = value[i],
                        8 => shared.gpus.vram_banks.h.write_vramcnt(value[i]),
                        9 => shared.gpus.vram_banks.i.write_vramcnt(value[i]),
                        _ => unreachable!(),
                    }
                }
            }

            0x04000280 => self.div.set_control(value.into_word()),
            0x04000290..=0x04000293 => self.div.set_numerator::<true>(value.into_word()),
            0x04000294..=0x04000297 => self.div.set_numerator::<false>(value.into_word()),
            0x04000298..=0x0400029B => self.div.set_denominator::<true>(value.into_word()),
            0x0400029C..=0x0400029F => self.div.set_denominator::<false>(value.into_word()),

            0x040002B0 => self.sqrt.set_control(value.into_word()),
            0x040002B8 => self.sqrt.set_param::<true>(value.into_word()),
            0x040002BC => self.sqrt.set_param::<false>(value.into_word()),

            0x04000304..=0x04000307 => shared.powcnt1 = value.into_word().into(),

            0x04000320..=0x040006A4 => self.logger.log_warn_once(format_debug!(
                "GPU3D not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04001004..=0x04001007 => {} // not real
            0x0400105C..=0x0400106B => {} // not real

            0x05000000..=0x05FFFFFF => {
                let addr = (addr - 0x05000000) % 0x800;
                if addr < 0x400 {
                    shared.gpus.a.palette[addr..addr + T].copy_from_slice(&value);
                } else {
                    let addr = addr - 0x400;
                    shared.gpus.b.palette[addr..addr + T].copy_from_slice(&value);
                }
            }

            0x07000000..=0x07FFFFFF => {
                let addr = (addr - 0x07000000) % 0x800;
                if addr < 0x400 {
                    shared.gpus.a.oam[addr..addr + T].copy_from_slice(&value);
                } else {
                    let addr = addr - 0x400;
                    shared.gpus.b.oam[addr..addr + T].copy_from_slice(&value);
                }
            }

            0xFFFF0000..=0xFFFF7FFF => {
                let addr = addr - 0xFFFF0000;
                self.bios[addr..addr + T].copy_from_slice(&value);
            }

            _ => {
                let mut success = false;
                if let Some(dma) = dma {
                    success = dma.write_slice::<T, Self>(addr, value);
                }

                success |= shared.gpus.vram_banks.write_slice::<T>(addr, value);

                if !success {
                    self.logger.log_error(format!(
                        "Invalid write {} byte(s) at address {:#010X}: {:#010X}",
                        T,
                        addr,
                        value.into_word()
                    ));
                }
            }
        }
    }
}
