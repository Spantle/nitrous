use models::{ExtKeyIn, IpcFifo, IpcSync, KeyInput, PostFlg, PowCnt1};

use super::{cart::Cartridge, gpus::Gpus};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Shared {
    pub cart: Cartridge,
    pub gpus: Gpus,
    pub psram: Vec<u8>,
    pub wram: Vec<u8>, // 32kb

    #[serde(skip)]
    pub keyinput: KeyInput, // 0x04000130
    #[serde(skip)]
    pub extkeyin: ExtKeyIn, // 0x04000136
    pub ipcsync: IpcSync, // 0x04000180
    pub ipcfifo: IpcFifo, // 0x04000184, 0x04000188, 0x04100000
    pub postflg: PostFlg, // 0x04000300 TODO: there's a tiny bit of logic behind this, and it's technically not "shared"
    pub powcnt1: PowCnt1, // 0x04000304
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            cart: Cartridge::default(),
            gpus: Gpus::default(),
            psram: vec![0; 1024 * 1024 * 4],
            wram: vec![0; 1024 * 32],

            keyinput: KeyInput::default(),
            extkeyin: ExtKeyIn::default(),
            ipcsync: IpcSync::default(),
            ipcfifo: IpcFifo::default(),
            postflg: PostFlg::default(),
            powcnt1: PowCnt1::default(),
        }
    }
}

impl Shared {
    pub fn new_fake() -> Self {
        Self {
            cart: Cartridge::default(),
            gpus: Gpus::new_fake(),
            psram: vec![0; 0],
            wram: vec![0; 0],

            keyinput: KeyInput::default(),
            extkeyin: ExtKeyIn::default(),
            ipcsync: IpcSync::default(),
            ipcfifo: IpcFifo::default(),
            postflg: PostFlg::default(),
            powcnt1: PowCnt1::default(),
        }
    }

    pub fn load_state(&mut self, shared: Self) {
        self.cart.load_state(shared.cart);
        self.gpus = shared.gpus;
        self.psram = shared.psram;
        self.wram = shared.wram;

        self.ipcsync = shared.ipcsync;
        self.ipcfifo = shared.ipcfifo;
        self.postflg = shared.postflg;
        self.powcnt1 = shared.powcnt1;
    }

    pub fn reset(&mut self) {
        self.cart.reset();
        self.gpus = Gpus::default();
        self.psram = vec![0; 1024 * 1024 * 4];
        self.wram = vec![0; 1024 * 32];

        self.ipcsync = IpcSync::default();
        self.ipcfifo = IpcFifo::default();
        self.powcnt1 = PowCnt1::default();
    }
}

pub mod models;
