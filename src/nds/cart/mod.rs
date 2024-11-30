use models::{AuxSpiCnt, Command, ExMem, RomCtrl};

use super::bus::{bus7::Bus7, bus9::Bus9};

mod models;

// TODO: we need to stream this

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Cartridge {
    #[serde(skip)]
    pub loaded: bool,
    #[serde(skip)]
    pub rom: Vec<u8>,
    #[serde(skip)]
    pub metadata: models::Metadata,

    pub romctrl: RomCtrl,
    pub auxspicnt: AuxSpiCnt,
    pub command: Command,
    pub exmemcnt: ExMem,
    pub exmemstat: ExMem,
}

impl Cartridge {
    pub fn load(&mut self, rom: Vec<u8>) -> bool {
        self.loaded = false;
        self.metadata = models::Metadata::default();

        self.rom = rom;
        self.metadata.parse(&self.rom);

        self.loaded = true;
        true
    }

    pub fn load_state(&mut self, cart: Self) {
        self.romctrl = cart.romctrl;
        self.auxspicnt = cart.auxspicnt;
        self.command = cart.command;
        self.exmemcnt = cart.exmemcnt;
        self.exmemstat = cart.exmemstat;
    }

    pub fn reset(&mut self) {
        self.romctrl = RomCtrl::default();
        self.auxspicnt = AuxSpiCnt::default();
        self.command = Command::default();
        self.exmemcnt = ExMem::default();
        self.exmemstat = ExMem::default();
    }

    pub fn clock(&mut self, bus9: &mut Bus9, bus7: &mut Bus7) {
        let interrupts = if self.exmemcnt.get_nds_slot_access_rights() {
            &mut bus9.interrupts
        } else {
            &mut bus7.interrupts
        };
        self.romctrl.clock(interrupts);
    }

    pub fn read_bus(&mut self) -> u32 {
        // TODO: i might need to return 0s if data is not actually ready
        // currently i just assume that the game won't read if data is not ready

        // TODO: this sucks
        let data = match self.command.get_command() {
            0xB7 => {
                let base_addr = self.command.get_read_address() as usize;
                let start_addr = base_addr + self.romctrl.words_read as usize * 4;
                // println!("Reading from {:X} {}", start_addr, self.romctrl.words_read);
                let data = self.rom[start_addr..start_addr + 4].to_vec();
                u32::from_le_bytes(data.try_into().unwrap())
            }
            0xB8 => 0x00000FC2,
            _ => 0,
        };

        self.romctrl.word_read();

        data
    }
}
