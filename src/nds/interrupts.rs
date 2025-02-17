use super::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Interrupts {
    pub me: InterruptMasterEnable, // 0x04000208, IME
    pub e: InterruptFlags,         // 0x04000210, IE
    pub f: InterruptFlags,         // 0x04000214, IF
}

impl Interrupts {
    pub fn is_requesting_interrupt(&self) -> bool {
        self.me.get_disable_all() && (self.e.0 & self.f.0) != 0
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct InterruptMasterEnable(u32);

impl From<u32> for InterruptMasterEnable {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl InterruptMasterEnable {
    pub fn value(&self) -> u32 {
        self.0
    }

    // actually inverted, 0 is disable all
    pub fn get_disable_all(&self) -> bool {
        self.0.get_bit(0)
    }
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct InterruptFlags(u32);

impl From<u32> for InterruptFlags {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl InterruptFlags {
    const LCD_VBLANK_OFFSET: u32 = 0;
    const LCD_HBLANK_OFFSET: u32 = 1;
    const LCD_VCOUNTER_MATCH_OFFSET: u32 = 2;
    const TIMER_OVERFLOW_START: u32 = 3;

    const DMA0_OFFSET: u32 = 8;
    const DMA1_OFFSET: u32 = 9;
    const DMA2_OFFSET: u32 = 10;
    const DMA3_OFFSET: u32 = 11;

    const IPCSYNC_OFFSET: u32 = 16;
    const IPC_SEND_FIFO_EMPTY_OFFSET: u32 = 17;
    const IPC_RECEIVE_FIFO_NOT_EMPTY_OFFSET: u32 = 18;
    const NDS_GAME_CARD_DATA_TRANSFER_COMPLETION_OFFSET: u32 = 19;

    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn write_and_ack(&mut self, value: u32) {
        self.0 &= !value;
    }

    pub fn set_lcd_vblank(&mut self, value: bool) {
        self.0.set_bit(Self::LCD_VBLANK_OFFSET, value);
    }

    pub fn set_lcd_hblank(&mut self, value: bool) {
        self.0.set_bit(Self::LCD_HBLANK_OFFSET, value);
    }

    pub fn set_lcd_vcounter_match(&mut self, value: bool) {
        self.0.set_bit(Self::LCD_VCOUNTER_MATCH_OFFSET, value);
    }

    pub fn falsy_set_timer_overflow(&mut self, timer: u8, value: bool) {
        let bit = Self::TIMER_OVERFLOW_START + (timer as u32);
        self.0.set_bit(bit, self.0.get_bit(bit) || value);
    }

    pub fn set_dma(&mut self, dma: u8, value: bool) {
        let bit = match dma {
            0 => Self::DMA0_OFFSET,
            1 => Self::DMA1_OFFSET,
            2 => Self::DMA2_OFFSET,
            3 => Self::DMA3_OFFSET,
            _ => unreachable!(),
        };
        self.0.set_bit(bit, value);
    }

    pub fn set_ipcsync(&mut self, value: bool) {
        self.0.set_bit(Self::IPCSYNC_OFFSET, value);
    }

    pub fn set_ipc_send_fifo_empty(&mut self, value: bool) {
        self.0.set_bit(Self::IPC_SEND_FIFO_EMPTY_OFFSET, value);
    }

    pub fn falsy_set_ipc_send_fifo_empty(&mut self, value: bool) {
        self.0.set_bit(
            Self::IPC_SEND_FIFO_EMPTY_OFFSET,
            self.0.get_bit(Self::IPC_SEND_FIFO_EMPTY_OFFSET) || value,
        );
    }

    pub fn set_ipc_receive_fifo_not_empty(&mut self, value: bool) {
        self.0
            .set_bit(Self::IPC_RECEIVE_FIFO_NOT_EMPTY_OFFSET, value);
    }

    pub fn falsy_set_ipc_receive_fifo_not_empty(&mut self, value: bool) {
        self.0.set_bit(
            Self::IPC_RECEIVE_FIFO_NOT_EMPTY_OFFSET,
            self.0.get_bit(Self::IPC_RECEIVE_FIFO_NOT_EMPTY_OFFSET) || value,
        );
    }

    pub fn falsy_set_nds_game_card_data_transfer_completion(&mut self, value: bool) {
        self.0.set_bit(
            Self::NDS_GAME_CARD_DATA_TRANSFER_COMPLETION_OFFSET,
            self.0
                .get_bit(Self::NDS_GAME_CARD_DATA_TRANSFER_COMPLETION_OFFSET)
                || value,
        );
    }
}
