use super::{logger, Bits, Bytes};

pub trait DMA {
    fn read_slice<const T: usize>(&self, addr: usize) -> Option<[u8; T]>;
    fn write_slice<const T: usize>(&mut self, addr: usize, value: [u8; T]) -> bool;
}

#[derive(Default)]
pub struct DMA9 {
    // source address, max 0FFFFFFE
    dma0sad: u32, // 0x040000B0
    dma1sad: u32, // 0x040000BC
    dma2sad: u32, // 0x040000C8
    dma3sad: u32, // 0x040000D4

    // destination address, max 0FFFFFFE
    dma0dad: u32, // 0x040000B4
    dma1dad: u32, // 0x040000C0
    dma2dad: u32, // 0x040000CC
    dma3dad: u32, // 0x040000D8

    // word count, 21 bits (200000h)
    dma0cnt_l: u32, // 0x040000B8
    dma1cnt_l: u32, // 0x040000C4
    dma2cnt_l: u32, // 0x040000D0
    dma3cnt_l: u32, // 0x040000DC

    // control
    dma0cnt_h: u16, // 0x040000BA
    dma1cnt_h: u16, // 0x040000C6
    dma2cnt_h: u16, // 0x040000D2
    dma3cnt_h: u16, // 0x040000DE

    // filldata
    dma0fill: u32, // 0x040000E0
    dma1fill: u32, // 0x040000E4
    dma2fill: u32, // 0x040000E8
    dma3fill: u32, // 0x040000EC
}

#[derive(Default)]
pub struct DMA7 {
    // source address, max 0FFFFFFF
    dma0sad: u32, // 0x040000B0, max 07FFFFFF
    dma1sad: u32, // 0x040000BC, max 07FFFFFF
    dma2sad: u32, // 0x040000C8, max 07FFFFFF
    dma3sad: u32, // 0x040000D4, max 07FFFFFF

    // destination address, max 0FFFFFFF
    dma0dad: u32, // 0x040000B4, max 07FFFFFF
    dma1dad: u32, // 0x040000C0, max 07FFFFFF
    dma2dad: u32, // 0x040000CC, max 07FFFFFF
    dma3dad: u32, // 0x040000D8, max 07FFFFFF

    // word count, 14 bits (4000h)
    dma0cnt_l: u16, // 0x040000B8
    dma1cnt_l: u16, // 0x040000C4
    dma2cnt_l: u16, // 0x040000D0
    dma3cnt_l: u16, // 0x040000DC, 16 bits (10000h)

    // control
    dma0cnt_h: u16, // 0x040000BA
    dma1cnt_h: u16, // 0x040000C6
    dma2cnt_h: u16, // 0x040000D2
    dma3cnt_h: u16, // 0x040000DE
}

impl DMA for DMA9 {
    fn read_slice<const T: usize>(&self, addr: usize) -> Option<[u8; T]> {
        match addr {
            0x040000B0 => Some(self.dma0sad.to_bytes::<T>()),
            0x040000B4 => Some(self.dma0dad.to_bytes::<T>()),
            0x040000B8 => Some(self.dma0cnt_l.to_bytes::<T>()),
            0x040000BA => Some(self.dma0cnt_h.to_bytes::<T>()),
            0x040000E0 => Some(self.dma0fill.to_bytes::<T>()),
            0x040000BC => Some(self.dma1sad.to_bytes::<T>()),
            0x040000C0 => Some(self.dma1dad.to_bytes::<T>()),
            0x040000C4 => Some(self.dma1cnt_l.to_bytes::<T>()),
            0x040000C6 => Some(self.dma1cnt_h.to_bytes::<T>()),
            0x040000E4 => Some(self.dma1fill.to_bytes::<T>()),
            0x040000C8 => Some(self.dma2sad.to_bytes::<T>()),
            0x040000CC => Some(self.dma2dad.to_bytes::<T>()),
            0x040000D0 => Some(self.dma2cnt_l.to_bytes::<T>()),
            0x040000D2 => Some(self.dma2cnt_h.to_bytes::<T>()),
            0x040000E8 => Some(self.dma2fill.to_bytes::<T>()),
            0x040000D4 => Some(self.dma3sad.to_bytes::<T>()),
            0x040000D8 => Some(self.dma3dad.to_bytes::<T>()),
            0x040000DC => Some(self.dma3cnt_l.to_bytes::<T>()),
            0x040000DE => Some(self.dma3cnt_h.to_bytes::<T>()),
            0x040000EC => Some(self.dma3fill.to_bytes::<T>()),
            _ => None,
        }
    }

    fn write_slice<const T: usize>(&mut self, addr: usize, value: [u8; T]) -> bool {
        let mut success = true;
        match addr {
            0x040000B0 => self.dma0sad = value.into_word(),
            0x040000B4 => self.dma0dad = value.into_word(),
            0x040000B8 => self.dma0cnt_l = value.into_word(),
            0x040000BA => {
                logger::warn(
                    logger::LogSource::DMA9,
                    format!(
                        "DMA0CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma0cnt_h = value.into_halfword();
            }
            0x040000BC => self.dma1sad = value.into_word(),
            0x040000C0 => self.dma1dad = value.into_word(),
            0x040000C4 => self.dma1cnt_l = value.into_word(),
            0x040000C6 => {
                logger::warn(
                    logger::LogSource::DMA9,
                    format!(
                        "DMA1CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma1cnt_h = value.into_halfword();
            }
            0x040000C8 => self.dma2sad = value.into_word(),
            0x040000CC => self.dma2dad = value.into_word(),
            0x040000D0 => self.dma2cnt_l = value.into_word(),
            0x040000D2 => {
                logger::warn(
                    logger::LogSource::DMA9,
                    format!(
                        "DMA2CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma2cnt_h = value.into_halfword();
            }
            0x040000D4 => self.dma3sad = value.into_word(),
            0x040000D8 => self.dma3dad = value.into_word(),
            0x040000DC => self.dma3cnt_l = value.into_word(),
            0x040000DE => {
                logger::warn(
                    logger::LogSource::DMA9,
                    format!(
                        "DMA3CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma3cnt_h = value.into_halfword();
            }
            0x040000E0 => self.dma0fill = value.into_word(),
            0x040000E4 => self.dma1fill = value.into_word(),
            0x040000E8 => self.dma2fill = value.into_word(),
            0x040000EC => self.dma3fill = value.into_word(),
            _ => success = false,
        };

        success
    }
}

impl DMA for DMA7 {
    fn read_slice<const T: usize>(&self, addr: usize) -> Option<[u8; T]> {
        match addr {
            0x040000B0 => Some(self.dma0sad.to_bytes::<T>()),
            0x040000B4 => Some(self.dma0dad.to_bytes::<T>()),
            0x040000B8 => Some(self.dma0cnt_l.to_bytes::<T>()),
            0x040000BA => Some(self.dma0cnt_h.to_bytes::<T>()),
            0x040000BC => Some(self.dma1sad.to_bytes::<T>()),
            0x040000C0 => Some(self.dma1dad.to_bytes::<T>()),
            0x040000C4 => Some(self.dma1cnt_l.to_bytes::<T>()),
            0x040000C6 => Some(self.dma1cnt_h.to_bytes::<T>()),
            0x040000C8 => Some(self.dma2sad.to_bytes::<T>()),
            0x040000CC => Some(self.dma2dad.to_bytes::<T>()),
            0x040000D0 => Some(self.dma2cnt_l.to_bytes::<T>()),
            0x040000D2 => Some(self.dma2cnt_h.to_bytes::<T>()),
            0x040000D4 => Some(self.dma3sad.to_bytes::<T>()),
            0x040000D8 => Some(self.dma3dad.to_bytes::<T>()),
            0x040000DC => Some(self.dma3cnt_l.to_bytes::<T>()),
            0x040000DE => Some(self.dma3cnt_h.to_bytes::<T>()),
            _ => None,
        }
    }

    fn write_slice<const T: usize>(&mut self, addr: usize, value: [u8; T]) -> bool {
        let mut success = true;
        match addr {
            0x040000B0 => self.dma0sad = value.into_word(),
            0x040000B4 => self.dma0dad = value.into_word(),
            0x040000B8 => self.dma0cnt_l = value.into_halfword(),
            0x040000BA => {
                logger::warn(
                    logger::LogSource::DMA7,
                    format!(
                        "DMA0CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma0cnt_h = value.into_halfword();
            }
            0x040000BC => self.dma1sad = value.into_word(),
            0x040000C0 => self.dma1dad = value.into_word(),
            0x040000C4 => self.dma1cnt_l = value.into_halfword(),
            0x040000C6 => {
                logger::warn(
                    logger::LogSource::DMA7,
                    format!(
                        "DMA1CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma1cnt_h = value.into_halfword();
            }
            0x040000C8 => self.dma2sad = value.into_word(),
            0x040000CC => self.dma2dad = value.into_word(),
            0x040000D0 => self.dma2cnt_l = value.into_halfword(),
            0x040000D2 => {
                logger::warn(
                    logger::LogSource::DMA7,
                    format!(
                        "DMA2CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma2cnt_h = value.into_halfword();
            }
            0x040000D4 => self.dma3sad = value.into_word(),
            0x040000D8 => self.dma3dad = value.into_word(),
            0x040000DC => self.dma3cnt_l = value.into_halfword(),
            0x040000DE => {
                logger::warn(
                    logger::LogSource::DMA7,
                    format!(
                        "DMA3CNT_H written: {:08X} Not implemented",
                        value.into_halfword()
                    ),
                );
                self.dma3cnt_h = value.into_halfword();
            }
            _ => success = false,
        };

        success
    }
}
