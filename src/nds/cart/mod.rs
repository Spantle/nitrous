mod models;

// TODO: we need to stream this

#[derive(Default)]
pub struct Cartridge {
    pub loaded: bool,
    pub rom: Vec<u8>,

    pub metadata: models::Metadata,
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
}
