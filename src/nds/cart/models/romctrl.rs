use crate::nds::{interrupts::Interrupts, Bits};

// I have to stop writing code at 2am, this can be done a lot better
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct RomCtrl {
    value: u32,

    cycles: u8,
    words_to_read: u32,
    just_finished: bool,
    pub words_read: u32,
}

impl RomCtrl {
    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn set(&mut self, data: u32) {
        let bit23 = self.value.get_bit(23);
        let bit29 = self.value.get_bit(29);
        let bit31 = self.value.get_bit(31);
        self.value = data;
        self.value.set_bit(23, bit23);
        self.value.set_bit(29, bit29 | self.value.get_bit(29));

        if self.value.get_bit(31) && !bit31 {
            self.words_read = 0;
            self.cycles = 10;

            let data_block_size = self.value.get_bits(24, 26);
            self.words_to_read = match data_block_size {
                0 => 0,
                7 => 4,
                _ => 0x100 << data_block_size,
            } / 4;

            self.set_data_word_ready(false);
        }
    }

    // data-word status
    pub fn set_data_word_ready(&mut self, value: bool) {
        self.value.set_bit(23, value);
    }

    // block start/status
    pub fn get_block_status(&self) -> bool {
        self.value.get_bit(31)
    }

    // returns true if finished
    pub fn word_read(&mut self) {
        self.words_read += 1;
        self.set_data_word_ready(false);

        // println!("Read word {}/{}", self.words_read, self.words_to_read);

        if self.words_read == self.words_to_read {
            self.value.set_bit(31, false);
            self.just_finished = true;
        } else {
            self.cycles = 10;
        }
    }

    pub fn clock(&mut self, interrupts: &mut Interrupts) {
        self.cycles = self.cycles.saturating_sub(1);

        self.set_data_word_ready(self.get_block_status() && self.cycles == 0);

        interrupts
            .f
            .falsy_set_nds_game_card_data_transfer_completion(self.just_finished);
        self.just_finished = false;
    }
}
