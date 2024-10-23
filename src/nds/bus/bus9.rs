use crate::nds::{
    arm::ArmKind,
    div::DividerUnit,
    dma::Dma,
    interrupts::Interrupts,
    logger::{self, Logger, LoggerTrait},
    shared::Shared,
    timers::Timers,
    Bits, Bytes,
};

use super::BusTrait;

pub struct Bus9 {
    logger: Logger,

    pub bios: Vec<u8>,
    pub interrupts: Interrupts,

    pub dma: Dma<Bus9>,
    pub timers: Timers,
    pub div: DividerUnit,
}

impl Default for Bus9 {
    fn default() -> Self {
        Self {
            logger: Logger(logger::LogSource::Bus9),

            bios: Vec::new(),
            interrupts: Interrupts::default(),

            dma: Dma::default(),
            timers: Timers::default(),
            div: DividerUnit::default(),
        }
    }
}

impl BusTrait for Bus9 {
    const KIND: ArmKind = ArmKind::Arm9;

    fn reset(&mut self) {
        self.interrupts = Interrupts::default();
        self.dma = Dma::default();
        self.timers = Timers::default();
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

    fn is_requesting_interrupt(&self) -> bool {
        self.interrupts.is_requesting_interrupt()
    }

    fn read_byte(&self, shared: &mut Shared, addr: u32) -> u8 {
        self.read_slice::<1>(shared, addr)[0]
    }
    fn read_halfword(&self, shared: &mut Shared, addr: u32) -> u16 {
        let bytes = self.read_slice::<2>(shared, addr);
        u16::from_le_bytes(bytes)
    }
    fn read_word(&self, shared: &mut Shared, addr: u32) -> u32 {
        let bytes = self.read_slice::<4>(shared, addr);
        u32::from_le_bytes(bytes)
    }

    fn write_byte(&mut self, shared: &mut Shared, addr: u32, value: u8) {
        self.write_slice::<1>(shared, addr, [value]);
    }
    fn write_halfword(&mut self, shared: &mut Shared, addr: u32, value: u16) {
        self.write_slice::<2>(shared, addr, value.to_le_bytes());
    }
    fn write_word(&mut self, shared: &mut Shared, addr: u32, value: u32) {
        self.write_slice::<4>(shared, addr, value.to_le_bytes());
    }

    #[inline(always)]
    fn read_slice<const T: usize>(&self, shared: &mut Shared, addr: u32) -> [u8; T] {
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
                self.logger.log_warn_once(format!(
                    "GPU feature not implemented (R{} {:#010X})",
                    T, addr
                ));
                bytes
            }
            0x04001020..=0x0400104F => {
                self.logger.log_warn_once(format!(
                    "GPU feature not implemented (R{} {:#010X})",
                    T, addr
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

            0x04000100..=0x04000101 => self.timers.get(0).get_counter().to_bytes::<T>(),
            0x04000102..=0x04000103 => self.timers.get(0).get_control().to_bytes::<T>(),
            0x04000104..=0x04000105 => self.timers.get(1).get_counter().to_bytes::<T>(),
            0x04000106..=0x04000107 => self.timers.get(1).get_control().to_bytes::<T>(),
            0x04000108..=0x04000109 => self.timers.get(2).get_counter().to_bytes::<T>(),
            0x0400010A..=0x0400010B => self.timers.get(2).get_control().to_bytes::<T>(),
            0x0400010C..=0x0400010D => self.timers.get(3).get_counter().to_bytes::<T>(),
            0x0400010E..=0x0400010F => self.timers.get(3).get_control().to_bytes::<T>(),

            0x04000130..=0x04000131 => shared.keyinput.value().to_bytes::<T>(),
            0x04000136..=0x04000137 => shared.extkeyin.value().to_bytes::<T>(),

            0x04000180..=0x04000183 => shared.ipcsync.value::<true>().to_bytes::<T>(),
            0x04000184..=0x04000185 => shared.ipcfifo.get_cnt::<true>().to_bytes::<T>(),

            0x040001A4..=0x040001A7 => shared.cart.romctrl.value().to_bytes::<T>(),

            0x04000204..=0x04000205 => shared.cart.exmemcnt.0.to_bytes::<T>(),

            0x04000208..=0x0400020B => self.interrupts.me.value().to_bytes::<T>(),
            0x04000210..=0x04000213 => self.interrupts.e.value().to_bytes::<T>(),
            0x04000214..=0x04000217 => self.interrupts.f.value().to_bytes::<T>(),

            0x04000240..=0x04000249 => {
                let len = T.min(shared.vramcnt.len());
                bytes[..len].copy_from_slice(&shared.vramcnt[..len]);
                bytes
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

            0x040002B0..=0x040002BF => {
                self.logger
                    .log_warn_once(format!("SQRT not implemented (R{} {:#010X})", T, addr));
                bytes
            }

            0x04000300 => shared.postflg.0.to_bytes::<T>(),
            0x04000304..=0x04000307 => shared.powcnt1.value().to_bytes::<T>(),

            0x04004000..=0x04004001 => bytes, // DSi Stuff, return nothing
            0x04004008..=0x0400400B => bytes, // DSi Stuff, return nothing

            0x04100000..=0x04100003 => shared.ipcfifo.receive::<true>().to_bytes::<T>(),
            0x04100010..=0x04100013 => shared.cart.read_bus().to_bytes::<T>(),

            0x06800000..=0x068A4000 => {
                let addr = addr - 0x06800000;
                bytes.copy_from_slice(&shared.vram_lcdc_alloc[addr..addr + T]);
                bytes
            }

            0x08000000..=0x0AFFFFFF => bytes, // gba slot, return nothing... for now?

            0xFFFF0000..=0xFFFF7FFF => {
                let addr = addr - 0xFFFF0000;
                bytes.copy_from_slice(&self.bios[addr..addr + T]);
                bytes
            }

            _ => {
                if let Some(bytes) = self.dma.read_slice::<T>(addr) {
                    return bytes;
                }

                self.logger.log_error(format!(
                    "Invalid read {} byte(s) at address {:#010X}",
                    T, addr
                ));
                bytes
            }
        }
    }

    #[inline(always)]
    fn write_slice<const T: usize>(&mut self, shared: &mut Shared, addr: u32, value: [u8; T]) {
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
            0x04000004..=0x04000005 => shared.gpus.dispstat = value.into_halfword().into(),

            0x04000008..=0x04000009 => shared.gpus.a.bgxcnt[0] = value.into_halfword().into(),
            0x0400000A..=0x0400000B => shared.gpus.a.bgxcnt[1] = value.into_halfword().into(),
            0x0400000C..=0x0400000D => shared.gpus.a.bgxcnt[2] = value.into_halfword().into(),
            0x0400000E..=0x0400000F => shared.gpus.a.bgxcnt[3] = value.into_halfword().into(),

            0x04001008..=0x04001009 => shared.gpus.b.bgxcnt[0] = value.into_halfword().into(),
            0x0400100A..=0x0400100B => shared.gpus.b.bgxcnt[1] = value.into_halfword().into(),
            0x0400100C..=0x0400100D => shared.gpus.b.bgxcnt[2] = value.into_halfword().into(),
            0x0400100E..=0x0400100F => shared.gpus.b.bgxcnt[3] = value.into_halfword().into(),

            0x04000010..=0x04000011 => shared.gpus.a.bghofs[0] = value.into_halfword(),
            0x04000012..=0x04000013 => shared.gpus.a.bgvofs[0] = value.into_halfword(),
            0x04000014..=0x04000015 => shared.gpus.a.bghofs[1] = value.into_halfword(),
            0x04000016..=0x04000017 => shared.gpus.a.bgvofs[1] = value.into_halfword(),
            0x04000018..=0x04000019 => shared.gpus.a.bghofs[2] = value.into_halfword(),
            0x0400001A..=0x0400001B => shared.gpus.a.bgvofs[2] = value.into_halfword(),
            0x0400001C..=0x0400001D => shared.gpus.a.bghofs[3] = value.into_halfword(),
            0x0400001E..=0x0400001F => shared.gpus.a.bgvofs[3] = value.into_halfword(),

            0x04001010..=0x04001011 => shared.gpus.b.bghofs[0] = value.into_halfword(),
            0x04001012..=0x04001013 => shared.gpus.b.bgvofs[0] = value.into_halfword(),
            0x04001014..=0x04001015 => shared.gpus.b.bghofs[1] = value.into_halfword(),
            0x04001016..=0x04001017 => shared.gpus.b.bgvofs[1] = value.into_halfword(),
            0x04001018..=0x04001019 => shared.gpus.b.bghofs[2] = value.into_halfword(),
            0x0400101A..=0x0400101B => shared.gpus.b.bgvofs[2] = value.into_halfword(),
            0x0400101C..=0x0400101D => shared.gpus.b.bghofs[3] = value.into_halfword(),
            0x0400101E..=0x0400101F => shared.gpus.b.bgvofs[3] = value.into_halfword(),

            0x04000020..=0x0400004F => self.logger.log_warn_once(format!(
                "GPU feature not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04001020..=0x0400104F => self.logger.log_warn_once(format!(
                "GPU feature not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04000050..=0x04000058 => self.logger.log_warn_once(format!(
                "Colour Special Effects not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04001050..=0x04001058 => self.logger.log_warn_once(format!(
                "Colour Special Effects not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x0400005C..=0x0400005F => {} // not real

            0x04000060..=0x04000061 => self.logger.log_warn_once(format!(
                "GPU3D not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04000064..=0x04000067 => self.logger.log_warn_once(format!(
                "DISPCAPCNT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x04000068..=0x0400006B => self.logger.log_warn_once(format!(
                "DISP_MMEM_FIFO not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x0400006C..=0x0400006D => self.logger.log_warn_once(format!(
                "MASTER_BRIGHT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),
            0x0400106C..=0x0400106D => self.logger.log_warn_once(format!(
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

            0x04000180..=0x04000183 => shared.ipcsync.set::<true>(value.into_word()),
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

            0x04000280 => self.div.set_control(value.into_word()),
            0x04000290..=0x04000293 => self.div.set_numerator::<true>(value.into_word()),
            0x04000294..=0x04000297 => self.div.set_numerator::<false>(value.into_word()),
            0x04000298..=0x0400029B => self.div.set_denominator::<true>(value.into_word()),
            0x0400029C..=0x0400029F => self.div.set_denominator::<false>(value.into_word()),

            0x040002B0..=0x040002BF => self.logger.log_warn_once(format!(
                "SQRT not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04000304..=0x04000307 => shared.powcnt1 = value.into_word().into(),
            0x04000240..=0x04000249 => {
                let len = T.min(shared.vramcnt.len());
                shared.vramcnt[..len].copy_from_slice(&value[..len]);
            }

            0x04001004..=0x04001007 => {} // not real
            0x0400105C..=0x0400106B => {} // not real

            0x05000000..=0x050003FF => {
                let addr = addr - 0x05000000;
                shared.gpus.a.palette[addr..addr + T].copy_from_slice(&value);
            }
            0x05000400..=0x050007FF => {
                let addr = addr - 0x05000400;
                shared.gpus.b.palette[addr..addr + T].copy_from_slice(&value);
            }

            0x06000000..=0x061FFFFF => {
                let addr = (addr - 0x06000000) % 0x80000;
                shared.gpus.a.bg_vram[addr..addr + T].copy_from_slice(&value);
            }
            0x06200000..=0x063FFFFF => {
                let addr = (addr - 0x06200000) % 0x20000;
                shared.gpus.b.bg_vram[addr..addr + T].copy_from_slice(&value);
            }
            0x06400000..=0x067FFFFF => self.logger.log_warn_once(format!(
                "OBJ VRAM not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x06800000..=0x068A4000 => {
                let addr = addr - 0x06800000;
                shared.vram_lcdc_alloc[addr..addr + T].copy_from_slice(&value);
            }

            0x07000000..=0x07FFFFFF => self.logger.log_warn_once(format!(
                "OAM not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0xFFFF0000..=0xFFFF7FFF => {
                let addr = addr - 0xFFFF0000;
                self.bios[addr..addr + T].copy_from_slice(&value);
            }

            _ => {
                let success = self.dma.write_slice::<T>(addr, value);
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
