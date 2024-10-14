use crate::nds::{
    gpus::gpu2d::{models::ColorPalette, Gpu2d},
    Bits,
};

type Size = (u32, u32, u32);

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    pub fn draw_background<const BG: u8>(&self) -> (Vec<u16>, u32, u32) {
        let mode = self.dispcnt.get_bg_mode();

        let bgcnt = &self.bgxcnt[BG as usize];
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
        let size = self.calculate_size::<BG>(); // TODO: use screen size to do stuff with the size of the screen lol

        let color_palette = bgcnt.get_color_palette();
        let mut pixels: Vec<u16> = vec![0; (size.0 * size.1) as usize];
        // iterate through every tile in the map
        for map_tile_i in 0..=1023 {
            let map_tile_address = (screen_base + map_tile_i * 2) as usize;
            let mut map_tile_bytes = [0; 2];
            map_tile_bytes.copy_from_slice(&self.bg_vram[map_tile_address..map_tile_address + 2]);
            let map_tile = u16::from_le_bytes(map_tile_bytes); // the tile in the map itself

            let tile_number = (map_tile as u32).get_bits(0, 9); // the ID of the tile pixel data
            let horizonal_flip = map_tile.get_bit(10);
            let vertical_flip = map_tile.get_bit(11);
            let palette_number = map_tile.get_bits(12, 15) as u32; // not used in 256/1

            match color_palette {
                ColorPalette::Is16x16 => {
                    // these are up here for performance
                    let map_tile_x = map_tile_i % 32 * 8;
                    let map_tile_y = map_tile_i / 32 * 8;
                    let palette_offset_address = palette_number << 4;

                    // tile data pixel in vram
                    let tile_address = (character_base + tile_number * 32) as usize;
                    let tile_bytes = &self.bg_vram[tile_address..tile_address + 32];
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
                            &self.palette[l_tile_palette_address..l_tile_palette_address + 2],
                        );
                        r_tile_bytes.copy_from_slice(
                            &self.palette[r_tile_palette_address..r_tile_palette_address + 2],
                        );
                        let mut l_color = u16::from_le_bytes(l_tile_bytes);
                        let mut r_color = u16::from_le_bytes(r_tile_bytes);

                        // MASSIVE NOTE: THESE ARE NOT REAL!!!! I SET THE TRANSPARENCY BIT IN THIS MODE BECAUSE I AM CHEATING!!!!
                        l_color.set_bit(15, tile_byte != 0 && l_tile_palette_i != 0);
                        r_color.set_bit(15, tile_byte != 0 && r_tile_palette_i != 0);

                        // TODO: these will probably all need to be adjusted for different map sizes
                        let l_pixel_x = tile_byte_i as u32 * 2 % 8;
                        let r_pixel_x = (tile_byte_i as u32 * 2 + 1) % 8;
                        let pixel_y = tile_byte_i as u32 * 2 / 8;
                        let l_pos = (map_tile_x + l_pixel_x) + (map_tile_y + pixel_y) * size.0;
                        let r_pos = (map_tile_x + r_pixel_x) + (map_tile_y + pixel_y) * size.0;

                        pixels[l_pos as usize] = l_color;
                        pixels[r_pos as usize] = r_color;
                    });
                }
                ColorPalette::Is256x1 => {
                    // these are up here for performance
                    let map_tile_x = map_tile_i % 32 * 8;
                    let map_tile_y = map_tile_i / 32 * 8;

                    let tile_address = (character_base + tile_number * 64) as usize;
                    let tile_bytes = &self.bg_vram[tile_address..tile_address + 64];
                    (0..tile_bytes.len()).for_each(|tile_byte_i| {
                        let tile_byte = tile_bytes[tile_byte_i]; // a pixel
                        let palette_address = tile_byte as usize * 2;
                        let mut tile_bytes = [0; 2];
                        tile_bytes
                            .copy_from_slice(&self.palette[palette_address..palette_address + 2]);
                        let mut color = u16::from_le_bytes(tile_bytes);
                        color.set_bit(15, tile_byte != 0); // MASSIVE NOTE: THIS IS NOT REAL!!!! I SET THE TRANSPARENCY BIT IN THIS MODE BECAUSE I AM CHEATING!!!!

                        // TODO: these will probably all need to be adjusted for different map sizes
                        let pixel_x = tile_byte_i as u32 % 8;
                        let pixel_y = tile_byte_i as u32 / 8;
                        let pos = (map_tile_x + pixel_x) + (map_tile_y + pixel_y) * size.0;

                        pixels[pos as usize] = color;
                    });
                }
            }
        }

        (pixels, size.0, size.1)
    }

    fn calculate_size<const BG: u8>(&self) -> Size {
        // TODO: MODE DETERMINES SIZE!!! WE ASSUME TEXT MODE FOR NOW
        let bgcnt = &self.bgxcnt[BG as usize];
        let bgcnt_size = bgcnt.get_screen_size();
        match bgcnt_size {
            // x, y, size (size = x*y/32?)
            0 => (256, 256, 2048),
            1 => (512, 256, 4096),
            2 => (256, 512, 4096),
            3 => (512, 512, 8192),
            _ => unreachable!(),
        }
    }
}