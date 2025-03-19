use crate::nds::{
    logger::{LogSource, Logger, LoggerTrait},
    Bits,
};

use super::models::{Mst, Offset};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct VramBank<const ID: u8> {
    mst: Mst,
    offset: Offset,
    enabled: bool,

    on_bus: bool,
    pub start: usize,
    pub end: usize,

    data: Vec<u8>,
}

impl<const ID: u8> Default for VramBank<ID> {
    fn default() -> Self {
        let mst = Mst::default();
        let offset = Offset::default();

        let (on_bus, start, end) = Self::map(mst, offset);

        let size = match ID {
            0..=3 => 128, // A, B, C, D
            4 => 64,      // E
            5..=6 => 16,  // F, G
            7 => 32,      // H
            8 => 16,      // I
            _ => unreachable!(),
        };

        Self {
            mst,
            offset,
            enabled: false,

            on_bus,
            start,
            end,

            data: vec![0; size * 1024],
        }
    }
}

impl<const ID: u8> VramBank<ID> {
    pub fn new_fake() -> Self {
        Self {
            mst: Mst::default(),
            offset: Offset::default(),
            enabled: false,

            on_bus: false,
            start: 0,
            end: 0,

            data: vec![0; 0],
        }
    }

    fn logger() -> Logger {
        Logger(LogSource::VramBank(ID))
    }

    fn map(mst: Mst, offset: Offset) -> (bool, usize, usize) {
        let offset = offset as usize;
        let offset0 = offset.get_bit(0) as usize;
        let offset1 = offset.get_bit(1) as usize;
        let (a, b, c) = match (ID, mst) {
            (0, Mst::A) => (true, 0x06800000, 0x0681FFFF),
            (1, Mst::A) => (true, 0x06820000, 0x0683FFFF),
            (2, Mst::A) => (true, 0x06840000, 0x0685FFFF),
            (3, Mst::A) => (true, 0x06860000, 0x0687FFFF),
            (4, Mst::A) => (true, 0x06880000, 0x0688FFFF),
            (5, Mst::A) => (true, 0x06890000, 0x06893FFF),
            (6, Mst::A) => (true, 0x06894000, 0x06897FFF),
            (7, Mst::A) => (true, 0x06898000, 0x0689FFFF),
            (8, Mst::A) => (true, 0x068A0000, 0x068A3FFF),

            (0..=3, Mst::B) => (
                true,
                0x06000000 + (0x20000 * offset),
                0x06000000 + (0x20000 * offset) + (1024 * 128),
            ),
            (4, Mst::B) => (true, 0x06000000, 0x06000000 + (1024 * 64)),
            (5..=6, Mst::B) => (
                true,
                0x06000000 + (0x4000 * offset0) + (0x10000 * offset1),
                0x06000000 + (0x4000 * offset0) + (0x10000 * offset1) + (1024 * 16),
            ),
            (7, Mst::B) => (true, 0x06200000, 0x06200000 + (1024 * 32)),
            (8, Mst::B) => (true, 0x06208000, 0x06208000 + (1024 * 16)),

            (0..=1, Mst::C) => (
                true,
                0x06400000 + (0x20000 * offset0),
                0x06400000 + (0x20000 * offset0) + (1024 * 128),
            ),
            (2..=3, Mst::C) => (
                true,
                0x06000000 + (0x20000 * offset0),
                0x06000000 + (0x20000 * offset0) + (1024 * 128),
            ),
            (4, Mst::C) => (true, 0x06400000, 0x06400000 + (1024 * 64)),
            (5..=6, Mst::C) => (
                true,
                0x06400000 + (0x4000 * offset0) + (0x10000 * offset1),
                0x06400000 + (0x4000 * offset0) + (0x10000 * offset1) + (1024 * 16),
            ),
            (7, Mst::C) => (false, 0, 3),
            (8, Mst::C) => (true, 0x06600000, 0x06600000 + (1024 * 16)),

            (0..=3, Mst::D) => (false, offset, offset),
            (4, Mst::D) => (false, 0, 3),
            (5..=6, Mst::D) => (false, offset0 + (offset1 * 4), offset0 + (offset1 * 4)),
            (8, Mst::D) => (false, 0, 0),

            (2, Mst::E) => (true, 0x06200000, 0x06200000 + (1024 * 128)),
            (3, Mst::E) => (true, 0x06600000, 0x06600000 + (1024 * 128)),
            (4, Mst::E) => (false, 0, 1),
            (5..=6, Mst::E) => {
                if offset == 0 {
                    (false, 0, 1)
                } else {
                    // offset = 1
                    (false, 2, 3)
                }
            }

            (5..=6, Mst::F) => (false, 0, 0),
            _ => unreachable!("invalid id/mst combination {}/{}", ID, mst as usize),
        };

        Self::logger().log_debug(format!(
            "Mapped VRAM bank {} to 0x{:08X} - 0x{:08X} {}",
            ID, b, c, a
        ));

        (a, b, c)
    }

    pub fn read_vramcnt(&self) -> u8 {
        let mut vramcnt: u8 = 0;
        vramcnt.set_bits(0, 2, self.mst as u8);
        vramcnt.set_bits(3, 4, self.offset as u8);
        vramcnt.set_bit(7, self.enabled);
        vramcnt
    }

    pub fn write_vramcnt(&mut self, vramcnt: u8) {
        self.mst = vramcnt.get_bits(0, 2).into();
        self.offset = vramcnt.get_bits(3, 4).into();
        self.enabled = vramcnt.get_bit(7);

        (self.on_bus, self.start, self.end) = Self::map(self.mst, self.offset);
    }

    pub fn read_slice<const T: usize>(&self, addr: usize) -> (bool, [u8; T]) {
        if !self.on_bus || !self.enabled {
            return (false, [0; T]);
        }

        if addr >= self.start && addr <= self.end {
            let start = (addr - self.start) % self.data.len();
            let end = start + T;

            let mut bytes = [0; T];
            bytes.copy_from_slice(&self.data[start..end]);
            return (true, bytes);
        }

        (false, [0; T])
    }

    pub fn write_slice<const T: usize>(&mut self, addr: usize, value: [u8; T]) -> bool {
        if !self.on_bus || !self.enabled {
            return false;
        }

        if addr >= self.start && (addr + T) <= self.end {
            let start = addr - self.start;
            let end = start + T;
            self.data[start..end].copy_from_slice(&value);
            return true;
        }

        false
    }

    pub fn read_virtual_slice<const T: usize>(&self, addr: usize) -> (bool, [u8; T]) {
        if self.on_bus || !self.enabled {
            return (false, [0; T]);
        }

        if addr <= self.data.len() {
            let start = addr;
            let end = start + T;

            let mut bytes = [0; T];
            bytes.copy_from_slice(&self.data[start..end]);
            return (true, bytes);
        }

        (false, [0; T])
    }
}
