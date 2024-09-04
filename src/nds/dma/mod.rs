mod models;

use models::DMA_Channel;

use super::{arm::ArmKind, bus::BusTrait, shared::Shared, Bits, Bytes};

#[allow(clippy::upper_case_acronyms)]
pub struct DMA<Bus: BusTrait> {
    channel: [DMA_Channel<Bus>; 4],
}

impl<Bus: BusTrait> Clone for DMA<Bus> {
    fn clone(&self) -> Self {
        Self {
            channel: [
                self.channel[0].clone(),
                self.channel[1].clone(),
                self.channel[2].clone(),
                self.channel[3].clone(),
            ],
        }
    }
}

impl<Bus: BusTrait> Default for DMA<Bus> {
    fn default() -> Self {
        Self {
            channel: [
                DMA_Channel::new(0),
                DMA_Channel::new(1),
                DMA_Channel::new(2),
                DMA_Channel::new(3),
            ],
        }
    }
}

impl<Bus: BusTrait> DMA<Bus> {
    pub fn read_slice<const T: usize>(&self, addr: usize) -> Option<[u8; T]> {
        match addr {
            0x040000B0 => Some(self.channel[0].dmasad.to_bytes::<T>()),
            0x040000B4 => Some(self.channel[0].dmadad.to_bytes::<T>()),
            0x040000B8 => Some(self.channel[0].dmacnt.get().to_bytes::<T>()),
            0x040000BA => Some(self.channel[0].dmacnt.get_h().to_bytes::<T>()),
            0x040000E0 => Some(self.channel[0].dmafill.to_bytes::<T>()),

            0x040000BC => Some(self.channel[1].dmasad.to_bytes::<T>()),
            0x040000C0 => Some(self.channel[1].dmadad.to_bytes::<T>()),
            0x040000C4 => Some(self.channel[1].dmacnt.get().to_bytes::<T>()),
            0x040000C6 => Some(self.channel[1].dmacnt.get_h().to_bytes::<T>()),
            0x040000E4 => Some(self.channel[1].dmafill.to_bytes::<T>()),

            0x040000C8 => Some(self.channel[2].dmasad.to_bytes::<T>()),
            0x040000CC => Some(self.channel[2].dmadad.to_bytes::<T>()),
            0x040000D0 => Some(self.channel[2].dmacnt.get().to_bytes::<T>()),
            0x040000D2 => Some(self.channel[2].dmacnt.get_h().to_bytes::<T>()),
            0x040000E8 => Some(self.channel[2].dmafill.to_bytes::<T>()),

            0x040000D4 => Some(self.channel[3].dmasad.to_bytes::<T>()),
            0x040000D8 => Some(self.channel[3].dmadad.to_bytes::<T>()),
            0x040000DC => Some(self.channel[3].dmacnt.get().to_bytes::<T>()),
            0x040000DE => Some(self.channel[3].dmacnt.get_h().to_bytes::<T>()),
            0x040000EC => Some(self.channel[3].dmafill.to_bytes::<T>()),
            _ => None,
        }
    }

    pub fn write_slice<const T: usize>(&mut self, addr: usize, value: [u8; T]) -> bool {
        let mut success = true;
        match addr {
            0x040000B0 => self.channel[0].dmasad = value.into_word(),
            0x040000B4 => self.channel[0].dmadad = value.into_word(),
            0x040000B8 => self.channel[0].update_cnt(value.into_word()),
            0x040000BA => self.channel[0].update_cnt_h(value.into_word() >> 16),
            0x040000E0 => self.channel[0].dmafill = value.into_word(),

            0x040000BC => self.channel[1].dmasad = value.into_word(),
            0x040000C0 => self.channel[1].dmadad = value.into_word(),
            0x040000C4 => self.channel[1].update_cnt(value.into_word()),
            0x040000C6 => self.channel[1].update_cnt_h(value.into_word() >> 16),
            0x040000E4 => self.channel[1].dmafill = value.into_word(),

            0x040000C8 => self.channel[2].dmasad = value.into_word(),
            0x040000CC => self.channel[2].dmadad = value.into_word(),
            0x040000D0 => self.channel[2].update_cnt(value.into_word()),
            0x040000D2 => self.channel[2].update_cnt_h(value.into_word() >> 16),
            0x040000E8 => self.channel[2].dmafill = value.into_word(),

            0x040000D4 => self.channel[3].dmasad = value.into_word(),
            0x040000D8 => self.channel[3].dmadad = value.into_word(),
            0x040000DC => self.channel[3].update_cnt(value.into_word()),
            0x040000DE => self.channel[3].update_cnt_h(value.into_word() >> 16),
            0x040000EC => self.channel[3].dmafill = value.into_word(),

            _ => success = false,
        };

        success
    }

    pub fn check_immediately(&mut self, bus: &mut Bus, shared: &mut Shared) -> Self {
        for channel in self.channel.iter_mut() {
            let matches = channel.dmacnt.get_dma_enable()
                && if Bus::KIND == ArmKind::Arm9 {
                    channel.dmacnt.get_dma9_start_timing()
                } else {
                    channel.dmacnt.get_dma7_start_timing()
                } == 0;

            if matches {
                channel.run(bus, shared);
            }
        }

        self.clone()
    }
}
