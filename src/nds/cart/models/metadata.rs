use crate::nds::logger::{self, Logger, LoggerTrait};

// this is mostly my stuff for my emulator
pub struct Metadata {
    logger: Logger,
    pub parse_error: bool,

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

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            logger: Logger(logger::LogSource::Cart),
            parse_error: false,
            game_title: String::new(),
            arm9_rom_offset: 0,
            arm9_entry_address: 0,
            arm9_load_address: 0,
            arm9_size: 0,
            arm7_rom_offset: 0,
            arm7_entry_address: 0,
            arm7_load_address: 0,
            arm7_size: 0,
        }
    }
}

impl Metadata {
    pub fn parse(&mut self, rom: &[u8]) -> bool {
        self.logger.log_info("=== Parsing ROM ===");
        self.logger
            .log_info(format!("ROM size: {} bytes", rom.len()));

        self.game_title = String::from_utf8_lossy(&rom[0x000..0x00C]).to_string();
        self.logger
            .log_info(format!("Game Title: {}", self.game_title));

        self.arm9_rom_offset = self.parse_u32(rom, 0x020);
        self.arm9_entry_address = self.parse_u32(rom, 0x024);
        self.arm9_load_address = self.parse_u32(rom, 0x028);
        self.arm9_size = self.parse_u32(rom, 0x02C);
        self.logger
            .log_info(format!("ARM9 ROM Offset: {:#010X}", self.arm9_rom_offset));
        self.logger.log_info(format!(
            "ARM9 Entry Address: {:#010X}",
            self.arm9_entry_address
        ));
        self.logger.log_info(format!(
            "ARM9 Load Address: {:#010X}",
            self.arm9_load_address
        ));
        self.logger
            .log_info(format!("ARM9 Size: {} bytes", self.arm9_size));

        self.arm7_rom_offset = self.parse_u32(rom, 0x030);
        self.arm7_entry_address = self.parse_u32(rom, 0x034);
        self.arm7_load_address = self.parse_u32(rom, 0x038);
        self.arm7_size = self.parse_u32(rom, 0x03C);
        self.logger
            .log_info(format!("ARM7 ROM Offset: {:#010X}", self.arm7_rom_offset));
        self.logger.log_info(format!(
            "ARM7 Entry Address: {:#010X}",
            self.arm7_entry_address
        ));
        self.logger.log_info(format!(
            "ARM7 Load Address: {:#010X}",
            self.arm7_load_address
        ));
        self.logger
            .log_info(format!("ARM7 Size: {} bytes", self.arm7_size));

        logger::info(logger::LogSource::Cart, "=== End ROM ===");

        if self.parse_error {
            logger::error(logger::LogSource::Cart, "Failed to parse ROM");
            return false;
        }

        true
    }

    fn parse_u32(&mut self, rom: &[u8], offset: usize) -> u32 {
        let value: [u8; 4] = rom[offset..offset + 4].try_into().unwrap_or_else(|e| {
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
