use crate::nds::{arm::ArmKind, bus::BusTrait, logger, shared::Shared, Bits};

// TODO: cycle timing
// TODO: GamePak DRQ
// TODO: special start timing
// TODO: IRQ upon end of word count
// TODO: maybe some edge cases? idk read gbatek lmao

pub struct DmaChannel<Bus: BusTrait> {
    _phantom: std::marker::PhantomData<Bus>,

    index: u8,

    // arm9: source address, max 0FFFFFFE
    // arm7: source address, max 07FFFFFF
    // 0x040000B0, 0x040000BC, 0x040000C8, 0x040000D4
    pub dmasad: u32,
    // arm9: destination address, max 0FFFFFFE
    // arm7: destination address, max 07FFFFFF
    // 0x040000B4, 0x040000C0, 0x040000CC, 0x040000D8
    pub dmadad: u32,
    // arm9: word count (21 bits - max 200000h), control
    // arm7: word count (14 bits - max 4000h), control
    // 0x040000B8/0x040000BA, 0x040000C4/0x040000C6, 0x040000D0/0x040000D2, 0x040000DC(arm7: 16 bits - max 10000h)/0x040000DE
    pub dmacnt: DmaCnt,
    // filldata, arm9 only
    // 0x040000E0, 0x040000E4, 0x040000E8, 0x040000EC
    pub dmafill: u32,

    internal_sad: u32,
    internal_dad: u32,
    internal_cnt_l: u32,
}

impl<Bus: BusTrait> Clone for DmaChannel<Bus> {
    fn clone(&self) -> Self {
        Self {
            _phantom: std::marker::PhantomData,

            index: self.index,

            dmasad: self.dmasad,
            dmadad: self.dmadad,
            dmacnt: self.dmacnt,
            dmafill: self.dmafill,

            internal_sad: self.internal_sad,
            internal_dad: self.internal_dad,
            internal_cnt_l: self.internal_cnt_l,
        }
    }
}

impl<Bus: BusTrait> DmaChannel<Bus> {
    pub fn new(index: u8) -> Self {
        Self {
            _phantom: std::marker::PhantomData,

            index,

            dmasad: 0,
            dmadad: 0,
            dmacnt: DmaCnt::default(),
            dmafill: 0,

            internal_sad: 0,
            internal_dad: 0,
            internal_cnt_l: 0,
        }
    }

    fn log_source(&self) -> logger::LogSource {
        if Bus::KIND == ArmKind::Arm9 {
            logger::LogSource::DMA9
        } else {
            logger::LogSource::DMA7
        }
    }

    pub fn update_cnt(&mut self, new_value: u32) {
        let old_enable = self.dmacnt.get_dma_enable();
        self.dmacnt.set(new_value);
        self.cnt_updated(old_enable);
    }

    pub fn update_cnt_h(&mut self, new_value: u32) {
        let old_enable = self.dmacnt.get_dma_enable();
        self.dmacnt.set_h(new_value);
        self.cnt_updated(old_enable);
    }

    fn cnt_updated(&mut self, old_enable: bool) {
        if old_enable || !self.dmacnt.get_dma_enable() {
            return;
        }

        let start_timing = if Bus::KIND == ArmKind::Arm9 {
            self.dmacnt.get_dma9_start_timing()
        } else {
            self.dmacnt.get_dma7_start_timing()
        };
        if start_timing != 0 {
            logger::warn(
                self.log_source(),
                format!(
                    "DMA{} has start timing {} which isn't supported",
                    self.index, start_timing
                ),
            );
        }

        self.internal_sad = self.dmasad;
        self.internal_dad = self.dmadad;
        self.internal_cnt_l = self.get_word_count();
        if self.internal_cnt_l == 0 {
            logger::error(
                self.log_source(),
                format!("DMA{} has 0 word count. Not implemented.", self.index),
            );
        }
    }

    pub fn run(&mut self, bus: &mut Bus, shared: &mut Shared) -> u32 {
        logger::debug(
            self.log_source(),
            format!(
                "DMA{} running {:08X},{:08X},{:08X},{:08X}",
                self.index,
                self.internal_sad,
                self.internal_dad,
                self.internal_cnt_l,
                self.dmacnt.get()
            ),
        );
        let is_32bit_transfer = self.dmacnt.get_dma_transfer_type();
        let offset_amount = if is_32bit_transfer { 4 } else { 2 };
        loop {
            if self.internal_cnt_l == 0 {
                // TODO: interrupt or something
                break;
            }
            self.internal_cnt_l -= 1;

            if is_32bit_transfer {
                let value = bus.read_word(shared, self.internal_sad);
                bus.write_word(shared, self.internal_dad, value);
            } else {
                let value = bus.read_halfword(shared, self.internal_sad);
                bus.write_halfword(shared, self.internal_dad, value);
            }

            match self.dmacnt.get_dest_addr_control() {
                0 | 3 => self.internal_dad = self.internal_dad.wrapping_add(offset_amount),
                1 => self.internal_dad = self.internal_dad.wrapping_sub(offset_amount),
                2 => {}
                _ => unreachable!(),
            }
            match self.dmacnt.get_source_addr_control() {
                0 => self.internal_sad = self.internal_sad.wrapping_add(offset_amount),
                1 => self.internal_sad = self.internal_sad.wrapping_sub(offset_amount),
                2 | 3 => {}
                _ => unreachable!(),
            }
        }

        if !self.dmacnt.get_dma_repeat() {
            self.dmacnt.set_dma_enable(false);
        } else {
            self.internal_cnt_l = self.get_word_count();
            if self.dmacnt.get_dest_addr_control() == 3 {
                self.internal_dad = self.dmadad;
            }
        }

        1 // TODO: this is wrong
    }

    fn get_word_count(&self) -> u32 {
        if Bus::KIND == ArmKind::Arm9 {
            let value = self.dmacnt.get().get_bits(0, 21);
            if value == 0 {
                0x200000
            } else {
                value
            }
        } else if self.index == 3 {
            let value = self.dmacnt.get().get_bits(0, 16);
            if value == 0 {
                0x10000
            } else {
                value
            }
        } else {
            let value = self.dmacnt.get().get_bits(0, 14);
            if value == 0 {
                0x4000
            } else {
                value
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct DmaCnt(u32);

impl DmaCnt {
    const DEST_ADDR_CONTROL_START: u32 = 16 + 5;
    const DEST_ADDR_CONTROL_END: u32 = 16 + 6;
    const SOURCE_ADDR_CONTROL_START: u32 = 16 + 7;
    const SOURCE_ADDR_CONTROL_END: u32 = 16 + 8;
    const DMA_REPEAT_OFFSET: u32 = 16 + 9;
    const DMA_TRANSFER_TYPE_OFFSET: u32 = 16 + 10;

    const DMA9_START_TIMING_START: u32 = 16 + 11;
    const DMA9_START_TIMING_END: u32 = 16 + 13;
    const DMA7_START_TIMING_START: u32 = 16 + 12;
    const DMA7_START_TIMING_END: u32 = 16 + 13;

    const DMA_ENABLE_OFFSET: u32 = 16 + 15;

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn set(&mut self, value: u32) {
        self.0 = value;
    }

    pub fn get_h(&self) -> u16 {
        self.0.get_bits(16, 31) as u16
    }

    pub fn set_h(&mut self, value: u32) {
        self.0.set_bits(16, 31, value);
    }

    pub fn get_dest_addr_control(&self) -> u32 {
        self.0
            .get_bits(Self::DEST_ADDR_CONTROL_START, Self::DEST_ADDR_CONTROL_END)
    }

    pub fn get_source_addr_control(&self) -> u32 {
        self.0.get_bits(
            Self::SOURCE_ADDR_CONTROL_START,
            Self::SOURCE_ADDR_CONTROL_END,
        )
    }

    pub fn get_dma_repeat(&self) -> bool {
        self.0.get_bit(Self::DMA_REPEAT_OFFSET)
    }

    pub fn get_dma_transfer_type(&self) -> bool {
        self.0.get_bit(Self::DMA_TRANSFER_TYPE_OFFSET)
    }

    pub fn get_dma9_start_timing(&self) -> u32 {
        self.0
            .get_bits(Self::DMA9_START_TIMING_START, Self::DMA9_START_TIMING_END)
    }

    pub fn get_dma7_start_timing(&self) -> u32 {
        self.0
            .get_bits(Self::DMA7_START_TIMING_START, Self::DMA7_START_TIMING_END)
    }

    pub fn get_dma_enable(&self) -> bool {
        self.0.get_bit(Self::DMA_ENABLE_OFFSET)
    }

    pub fn set_dma_enable(&mut self, value: bool) {
        self.0.set_bit(Self::DMA_ENABLE_OFFSET, value);
    }
}
