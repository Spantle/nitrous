use super::logger;

// TODO: we need to stream this

#[derive(Default)]
pub struct Cartridge {
    pub loaded: bool,
    pub parse_error: bool,
    pub rom: Vec<u8>,

    pub game_title: String,

    pub arm9_rom_offset: u32,
    pub arm9_entry_address: u32,
    pub arm9_load_address: u32,
    pub arm9_size: u32,

    pub arm7_rom_offset: u32,
    pub arm7_entry_address: u32,
    pub arm7_load_address: u32,
    pub arm7_size: u32,
}

impl Cartridge {
    pub fn load(&mut self, rom: Vec<u8>) -> bool {
        self.loaded = false;
        self.parse_error = false;

        self.rom = rom;

        logger::info(logger::LogSource::Cart, "=== Parsing ROM ===");
        logger::info(
            logger::LogSource::Cart,
            format!("ROM size: {} bytes", self.rom.len()),
        );

        self.game_title = String::from_utf8_lossy(&self.rom[0x000..0x00C]).to_string();
        logger::info(
            logger::LogSource::Cart,
            format!("Game Title: {}", self.game_title),
        );

        self.arm9_rom_offset = self.parse_u32(0x020);
        self.arm9_entry_address = self.parse_u32(0x024);
        self.arm9_load_address = self.parse_u32(0x028);
        self.arm9_size = self.parse_u32(0x02C);
        logger::info(
            logger::LogSource::Cart,
            format!("ARM9 ROM Offset: {:#010X}", self.arm9_rom_offset),
        );
        logger::info(
            logger::LogSource::Cart,
            format!("ARM9 Entry Address: {:#010X}", self.arm9_entry_address),
        );
        logger::info(
            logger::LogSource::Cart,
            format!("ARM9 Load Address: {:#010X}", self.arm9_load_address),
        );
        logger::info(
            logger::LogSource::Cart,
            format!("ARM9 Size: {} bytes", self.arm9_size),
        );

        self.arm7_rom_offset = self.parse_u32(0x030);
        self.arm7_entry_address = self.parse_u32(0x034);
        self.arm7_load_address = self.parse_u32(0x038);
        self.arm7_size = self.parse_u32(0x03C);
        logger::info(
            logger::LogSource::Cart,
            format!("ARM7 ROM Offset: {:#010X}", self.arm7_rom_offset),
        );
        logger::info(
            logger::LogSource::Cart,
            format!("ARM7 Entry Address: {:#010X}", self.arm7_entry_address),
        );
        logger::info(
            logger::LogSource::Cart,
            format!("ARM7 Load Address: {:#010X}", self.arm7_load_address),
        );
        logger::info(
            logger::LogSource::Cart,
            format!("ARM7 Size: {} bytes", self.arm7_size),
        );

        logger::info(logger::LogSource::Cart, "=== End ROM ===");

        if self.parse_error {
            logger::error(logger::LogSource::Cart, "Failed to parse ROM");
            return false;
        }

        self.loaded = true;
        true
    }

    fn parse_u32(&mut self, offset: usize) -> u32 {
        let value: [u8; 4] = self.rom[offset..offset + 4].try_into().unwrap_or_else(|e| {
            self.parse_error = true;
            logger::error(
                logger::LogSource::Cart,
                format!("Failed to parse u32 at offset {:#010X}: {}", offset, e),
            );
            [0; 4]
        });
        u32::from_le_bytes(value)
    }
}
