use crate::nds::Bits;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct Command(pub u64);

impl Command {
    // len is bytes
    pub fn update(&mut self, addr: usize, len: usize, data: u32) {
        let addr = 7 - addr;
        let data = data as u64;
        let mask = (1 << (len * 8)) - 1;
        let shift = (addr & 7) * 8;
        let mask = mask << shift;
        let mask = !mask;
        let data = data << shift;
        self.0 &= mask;
        self.0 |= data;

        // logger::debug(
        //     logger::LogSource::Cart,
        //     logger::format_debug!("Appended to command: {} {:#018X}", data, self.0),
        // );
    }

    pub fn get_command(&self) -> u8 {
        self.0.get_bits(56, 63) as u8
    }

    pub fn get_read_address(&self) -> u32 {
        // TODO: check the b7 command, there are edgecases for this
        self.0.get_bits(24, 55) as u32
    }
}
