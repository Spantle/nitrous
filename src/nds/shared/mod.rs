use models::{IpcFifo, IpcSync, KeyInput, PowCnt1};

use super::{
    bus::{bus7::Bus7, bus9::Bus9},
    cartridge::Cartridge,
    dma::Dma,
    gpus::Gpus,
};

pub struct Shared {
    pub cart: Cartridge,
    pub gpus: Gpus,
    pub psram: Vec<u8>,
    pub wram: Vec<u8>, // 32kb

    pub keyinput: KeyInput, // 0x04000130
    pub vramcnt: [u8; 10],  // 0x04000240 - 0x04000249, 0x04000247 is wramcnt
    pub ipcsync: IpcSync,   // 0x04000180
    pub ipcfifo: IpcFifo,   // 0x04000184, 0x04000188, 0x04100000
    pub powcnt1: PowCnt1,   // 0x04000304

    pub dma9: Dma<Bus9>,
    pub dma7: Dma<Bus7>,

    pub vram_lcdc_alloc: Vec<u8>, // 0x06800000
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            cart: Cartridge::default(),
            gpus: Gpus::default(),
            psram: vec![0; 1024 * 1024 * 4],
            wram: vec![0; 1024 * 32],

            keyinput: KeyInput::default(),
            vramcnt: [0; 10],
            ipcsync: IpcSync::default(),
            ipcfifo: IpcFifo::default(),
            powcnt1: PowCnt1::default(),

            dma9: Dma::default(),
            dma7: Dma::default(),

            vram_lcdc_alloc: vec![0; 1024 * 656],
        }
    }
}

impl Shared {
    pub fn new_fake() -> Self {
        Self {
            cart: Cartridge::default(),
            gpus: Gpus::default(),
            psram: vec![0; 0],
            wram: vec![0; 0],

            keyinput: KeyInput::default(),
            vramcnt: [0; 10],
            ipcsync: IpcSync::default(),
            ipcfifo: IpcFifo::default(),
            powcnt1: PowCnt1::default(),

            dma9: Dma::default(),
            dma7: Dma::default(),

            vram_lcdc_alloc: vec![0; 0],
        }
    }

    pub fn reset(&mut self) {
        self.gpus = Gpus::default();
        self.psram = vec![0; 1024 * 1024 * 4];
        self.wram = vec![0; 1024 * 32];

        self.vramcnt = [0; 10];
        self.ipcsync = IpcSync::default();
        self.ipcfifo = IpcFifo::default();
        self.powcnt1 = PowCnt1::default();

        self.dma9 = Dma::default();
        self.dma7 = Dma::default();

        self.vram_lcdc_alloc = vec![0; 1024 * 656];
    }
}

pub mod models;
