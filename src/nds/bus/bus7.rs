use crate::nds::{
    arm::ArmKind,
    dma::Dma,
    interrupts::Interrupts,
    logger::{self, Logger, LoggerTrait},
    shared::Shared,
    timers::Timers,
    Bits, Bytes,
};

use super::BusTrait;

pub struct Bus7 {
    logger: Logger,

    pub bios: Vec<u8>,
    pub interrupts: Interrupts,

    pub dma: Dma<Bus7>,
    pub timers: Timers,
    pub wram7: Vec<u8>, // 64kb
}

impl Default for Bus7 {
    fn default() -> Bus7 {
        Bus7 {
            logger: Logger(logger::LogSource::Bus7),

            bios: vec![],
            interrupts: Interrupts::default(),

            dma: Dma::default(),
            timers: Timers::default(),
            wram7: vec![0; 1024 * 64],
        }
    }
}

impl BusTrait for Bus7 {
    const KIND: ArmKind = ArmKind::Arm7;

    fn reset(&mut self) {
        self.interrupts = Interrupts::default();
        self.dma = Dma::default();
        self.timers = Timers::default();
        self.wram7 = vec![0; 1024 * 64];
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
            0x00000000..=0x00003FFF => {
                bytes.copy_from_slice(&self.bios[addr..addr + T]);
                bytes
            }

            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 0x400000;
                bytes.copy_from_slice(&shared.psram[addr..addr + T]);
                bytes
            }

            0x03000000..=0x037FFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                bytes.copy_from_slice(&shared.wram[addr..addr + T]);
                bytes
            }
            0x03800000..=0x03FFFFFF => {
                let addr = (addr - 0x03800000) % 0x10000;
                bytes.copy_from_slice(&self.wram7[addr..addr + T]);
                bytes
            }

            0x04000004..=0x04000005 => shared.gpus.dispstat.value().to_bytes::<T>(),

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
            0x04000138 => {
                self.logger
                    .log_warn_once(format!("RTC not implemented (R{} {:#010X})", T, addr));
                bytes
            }

            0x04000180..=0x04000183 => shared.ipcsync.value::<false>().to_bytes::<T>(),
            0x04000184..=0x04000187 => shared.ipcfifo.get_cnt::<false>().to_bytes::<T>(),

            0x040001C0..=0x040001C3 => {
                self.logger
                    .log_warn_once(format!("SPI not implemented (R{} {:#010X})", T, addr));
                bytes
            }

            0x04000208..=0x0400020B => self.interrupts.me.value().to_bytes::<T>(),
            0x04000210..=0x04000213 => self.interrupts.e.value().to_bytes::<T>(),
            0x04000214..=0x04000217 => self.interrupts.f.value().to_bytes::<T>(),

            0x04000304..=0x04000307 => shared.powcnt1.value().to_bytes::<T>(),

            0x04000400..=0x0400051F => {
                self.logger.log_warn_once(format!(
                    "Sound channels not implemented (R{} {:#010X})",
                    T, addr
                ));
                bytes
            }

            0x04100000..=0x04100003 => shared.ipcfifo.receive::<false>().to_bytes::<T>(),

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
            0x02000000..=0x02FFFFFF => {
                let addr = (addr - 0x02000000) % 0x400000;
                shared.psram[addr..addr + T].copy_from_slice(&value);
            }

            0x03000000..=0x037FFFFF => {
                let addr = (addr - 0x03000000) % 0x8000;
                shared.wram[addr..addr + T].copy_from_slice(&value);
            }
            0x03800000..=0x03FFFFFF => {
                let addr = (addr - 0x03800000) % 0x10000;
                self.wram7[addr..addr + T].copy_from_slice(&value);
            }

            0x04000004..=0x04000005 => shared.gpus.dispstat = value.into_halfword().into(),

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

            0x04000100..=0x04000101 => self.timers.get_mut(0).set_l(value.into_halfword()),
            0x04000102..=0x04000103 => self.timers.get_mut(0).set_h(value.into_halfword()),
            0x04000104..=0x04000105 => self.timers.get_mut(1).set_l(value.into_halfword()),
            0x04000106..=0x04000107 => self.timers.get_mut(1).set_h(value.into_halfword()),
            0x04000108..=0x04000109 => self.timers.get_mut(2).set_l(value.into_halfword()),
            0x0400010A..=0x0400010B => self.timers.get_mut(2).set_h(value.into_halfword()),
            0x0400010C..=0x0400010D => self.timers.get_mut(3).set_l(value.into_halfword()),
            0x0400010E..=0x0400010F => self.timers.get_mut(3).set_h(value.into_halfword()),

            0x04000134..=0x04000135 => {} // Debug RCNT, doesn't really do anything apparently
            0x04000138 => self.logger.log_warn_once(format!(
                "RTC not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04000180..=0x04000183 => shared.ipcsync.set::<false>(value.into_word()),
            0x04000184..=0x04000187 => shared
                .ipcfifo
                .set_cnt::<false>(&mut self.interrupts, value.into_word()),
            0x04000188..=0x0400018B => shared.ipcfifo.send::<false>(value.into_word()),

            0x040001C0..=0x040001C3 => self.logger.log_warn_once(format!(
                "SPI not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

            0x04000208..=0x0400020B => self.interrupts.me = value.into_word().into(),
            0x04000210..=0x04000213 => self.interrupts.e = value.into_word().into(),
            0x04000214..=0x04000217 => self.interrupts.f.write_and_ack(value.into_word()),

            0x04000304..=0x04000307 => shared.powcnt1 = value.into_word().into(),

            0x04000400..=0x0400051F => self.logger.log_warn_once(format!(
                "Sound channels not implemented (W{} {:#010X}:{:#010X})",
                T,
                addr,
                value.into_word()
            )),

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
        };
    }
}
