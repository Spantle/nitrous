use crate::nds::{
    gpus::{
        gpu2d::{models::ColorPalette, BackgroundResult, Gpu2d},
        vram::{VirtualLocation, VramBanks},
    },
    Bits, IfElse,
};

struct Size {
    pub x: usize,
    pub y: usize,
    pub quadrants: Vec<usize>,
}

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    pub fn render_background<const BG: u8>(&self, vram_banks: &VramBanks) -> BackgroundResult {
        let mode = self.dispcnt.get_bg_mode();

        let bgcnt = &self.bgxcnt[BG as usize];
        let bg_vram_base = if ENGINE_A { 0x06000000 } else { 0x06200000 };
        let character_base = if ENGINE_A {
            bgcnt.get_character_base_block() as u32 * (1024 * 16)
                + self.dispcnt.get_character_base() * (1024 * 64)
        } else {
            bgcnt.get_character_base_block() as u32 * (1024 * 16)
        };
        let screen_base = if ENGINE_A {
            bgcnt.get_screen_base_block() as u32 * (1024 * 2)
                + self.dispcnt.get_screen_base() * (1024 * 64)
        } else {
            bgcnt.get_screen_base_block() as u32 * (1024 * 2)
        };
        let size = self.calculate_size::<BG>();

        let color_palette = bgcnt.get_color_palette(self.dispcnt.get_bg_extended_palettes());
        let ext_palette_slot_offset = match (BG, bgcnt.get_ext_palette_slot()) {
            (0, false) => 0,
            (0, true) => 1024 * 16,
            (1, false) => 1024 * 8,
            (1, true) => 1024 * 24,
            (2, _) => 1024 * 16,
            (3, _) => 1024 * 24,
            _ => 0,
        };
        let mut pixels: Vec<Vec<u16>> = vec![vec![0; size.y]; size.x];
        // TODO: this could be done better, a world without this silly quadrant system and just good maths, but I really don't care right now. it's so late
        size.quadrants.iter().for_each(|quadrant| {
            let map_tile_i_offset = quadrant * 32 * 32;
            let map_tile_x_offset = quadrant % 2 * 256;
            let map_tile_y_offset = quadrant / 2 * 256;

            // iterate through every tile in the map
            (0..32).for_each(|map_tile_x| {
                let map_pixel_x = (map_tile_x * 8) + map_tile_x_offset;

                (0..32).for_each(|map_tile_y| {
                    let map_pixel_y = (map_tile_y * 8) + map_tile_y_offset;

                    let map_tile_i = ((map_tile_y * 32 + map_tile_x) + map_tile_i_offset) as u32;
                    let map_tile_address = (screen_base + map_tile_i * 2) as usize;
                    let map_tile_bytes = vram_banks
                        .read_slice::<2>(bg_vram_base + map_tile_address)
                        .unwrap(); // TODO: this might need to be unwrap_or
                    let map_tile = u16::from_le_bytes(map_tile_bytes); // the tile in the map itself

                    let tile_number = (map_tile as u32).get_bits(0, 9); // the ID of the tile pixel data
                    let horizontal_flip = map_tile.get_bit(10);
                    let vertical_flip = map_tile.get_bit(11);

                    let palette_number = map_tile.get_bits(12, 15) as u32; // not used in 256/1

                    match color_palette {
                        ColorPalette::Is16x16 => {
                            // this is up here for performance
                            let palette_offset_address = palette_number << 4;

                            // tile data pixel in vram
                            let tile_address = (character_base + tile_number * 32) as usize;
                            let tile_bytes =
                                vram_banks.read_slice::<32>(bg_vram_base + tile_address);
                            let tile_bytes = tile_bytes.unwrap(); // TODO: this might need to be unwrap_or

                            // iterate through each byte in the pixel data
                            (0..tile_bytes.len()).for_each(|tile_byte_i| {
                                let tile_byte = tile_bytes[tile_byte_i]; // two pixels
                                let l_tile_palette_i = tile_byte.get_bits(0, 3) as u32;
                                let r_tile_palette_i = tile_byte.get_bits(4, 7) as u32;

                                let l_tile_palette_address =
                                    ((l_tile_palette_i | palette_offset_address) * 2) as usize;
                                let r_tile_palette_address =
                                    ((r_tile_palette_i | palette_offset_address) * 2) as usize;

                                let mut l_tile_bytes = [0; 2];
                                let mut r_tile_bytes = [0; 2];
                                l_tile_bytes.copy_from_slice(
                                    &self.palette
                                        [l_tile_palette_address..l_tile_palette_address + 2],
                                );
                                r_tile_bytes.copy_from_slice(
                                    &self.palette
                                        [r_tile_palette_address..r_tile_palette_address + 2],
                                );
                                let mut l_color = u16::from_le_bytes(l_tile_bytes);
                                let mut r_color = u16::from_le_bytes(r_tile_bytes);

                                // MASSIVE NOTE: THESE ARE NOT REAL!!!! I SET THE TRANSPARENCY BIT IN THIS MODE BECAUSE I AM CHEATING!!!!
                                l_color.set_bit(15, tile_byte != 0 && l_tile_palette_i != 0);
                                r_color.set_bit(15, tile_byte != 0 && r_tile_palette_i != 0);

                                let l_tile_pixel_x = tile_byte_i * 2 % 8;
                                let l_tile_pixel_x_flipped = 7 - l_tile_pixel_x;
                                let r_tile_pixel_x = (tile_byte_i * 2 + 1) % 8;
                                let r_tile_pixel_x_flipped = 7 - r_tile_pixel_x;
                                let tile_pixel_y = tile_byte_i * 2 / 8;
                                let tile_pixel_y_flipped = 7 - tile_pixel_y;

                                // TODO: these will probably all need to be adjusted for different map sizes
                                let l_tile_pixel_x =
                                    horizontal_flip.if_else(l_tile_pixel_x_flipped, l_tile_pixel_x);
                                let r_tile_pixel_x =
                                    horizontal_flip.if_else(r_tile_pixel_x_flipped, r_tile_pixel_x);
                                let tile_pixel_y =
                                    vertical_flip.if_else(tile_pixel_y_flipped, tile_pixel_y);
                                let l_pixel_x = map_pixel_x + l_tile_pixel_x;
                                let r_pixel_x = map_pixel_x + r_tile_pixel_x;
                                let pixel_y = map_pixel_y + tile_pixel_y;

                                pixels[l_pixel_x][pixel_y] = l_color;
                                pixels[r_pixel_x][pixel_y] = r_color;
                            });
                        }
                        ColorPalette::Is256x1 => {
                            let tile_address = (character_base + tile_number * 64) as usize;
                            let tile_bytes =
                                vram_banks.read_slice::<64>(bg_vram_base + tile_address);
                            let tile_bytes = tile_bytes.unwrap(); // TODO: this might need to be unwrap_or
                            (0..tile_bytes.len()).for_each(|tile_byte_i| {
                                let tile_byte = tile_bytes[tile_byte_i]; // a pixel
                                let palette_address = tile_byte as usize * 2;
                                let mut tile_bytes = [0; 2];
                                tile_bytes.copy_from_slice(
                                    &self.palette[palette_address..palette_address + 2],
                                );
                                let mut color = u16::from_le_bytes(tile_bytes);
                                color.set_bit(15, tile_byte != 0); // MASSIVE NOTE: THIS IS NOT REAL!!!! I SET THE TRANSPARENCY BIT IN THIS MODE BECAUSE I AM CHEATING!!!!

                                let tile_pixel_x = tile_byte_i % 8;
                                let tile_pixel_x_flipped = 7 - tile_pixel_x;
                                let tile_pixel_y = tile_byte_i / 8;
                                let tile_pixel_y_flipped = 7 - tile_pixel_y;

                                // TODO: these will probably all need to be adjusted for different map sizes
                                let tile_pixel_x =
                                    horizontal_flip.if_else(tile_pixel_x_flipped, tile_pixel_x);
                                let tile_pixel_y =
                                    vertical_flip.if_else(tile_pixel_y_flipped, tile_pixel_y);
                                let pixel_x = map_pixel_x + tile_pixel_x;
                                let pixel_y = map_pixel_y + tile_pixel_y;

                                pixels[pixel_x][pixel_y] = color;
                            });
                        }
                        ColorPalette::Is256x16 => {
                            // this is up here for performance
                            let palette_offset_address = palette_number * 2 * 256;

                            let tile_address = (character_base + tile_number * 64) as usize;
                            let tile_bytes =
                                vram_banks.read_slice::<64>(bg_vram_base + tile_address);
                            let tile_bytes = tile_bytes.unwrap(); // TODO: this might need to be unwrap_or
                            (0..tile_bytes.len()).for_each(|tile_byte_i| {
                                let tile_byte = tile_bytes[tile_byte_i]; // a pixel
                                let palette_address = (ext_palette_slot_offset
                                    + ((tile_byte as u32) * 2 + palette_offset_address))
                                    as usize;
                                let virtual_location = match ENGINE_A {
                                    true => VirtualLocation::BgExtendedPaletteA,
                                    false => VirtualLocation::BgExtendedPaletteB,
                                };
                                let tile_bytes = vram_banks
                                    .read_virtual_slice::<2>(virtual_location, palette_address)
                                    .unwrap(); // TODO: this might need to be unwrap_or
                                let mut color = u16::from_le_bytes(tile_bytes);
                                color.set_bit(15, tile_byte != 0); // MASSIVE NOTE: THIS IS NOT REAL!!!! I SET THE TRANSPARENCY BIT IN THIS MODE BECAUSE I AM CHEATING!!!!

                                let tile_pixel_x = tile_byte_i % 8;
                                let tile_pixel_x_flipped = 7 - tile_pixel_x;
                                let tile_pixel_y = tile_byte_i / 8;
                                let tile_pixel_y_flipped = 7 - tile_pixel_y;

                                // TODO: these will probably all need to be adjusted for different map sizes
                                let tile_pixel_x =
                                    horizontal_flip.if_else(tile_pixel_x_flipped, tile_pixel_x);
                                let tile_pixel_y =
                                    vertical_flip.if_else(tile_pixel_y_flipped, tile_pixel_y);
                                let pixel_x = map_pixel_x + tile_pixel_x;
                                let pixel_y = map_pixel_y + tile_pixel_y;

                                pixels[pixel_x][pixel_y] = color;
                            });
                        }
                    }
                });
            });
        });

        (pixels, true)
    }

    fn calculate_size<const BG: u8>(&self) -> Size {
        // TODO: MODE DETERMINES SIZE!!! WE ASSUME TEXT MODE FOR NOW
        let bgcnt = &self.bgxcnt[BG as usize];
        let bgcnt_size = bgcnt.get_screen_size();
        match bgcnt_size {
            0 => Size {
                x: 256,
                y: 256,
                quadrants: vec![0],
            },
            1 => Size {
                x: 512,
                y: 256,
                quadrants: vec![0, 1],
            },
            2 => Size {
                x: 256,
                y: 512,
                quadrants: vec![0, 2],
            },
            3 => Size {
                x: 512,
                y: 512,
                quadrants: vec![0, 1, 2, 3],
            },
            _ => unreachable!(),
        }
    }
}
