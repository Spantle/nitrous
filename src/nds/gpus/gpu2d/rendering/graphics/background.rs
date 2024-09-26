use crate::nds::{
    gpus::gpu2d::{models::ColorPalette, Gpu2d},
    Bits,
};

type Size = (u32, u32, u32);
const COLOUR_MULT: f32 = 255.0 / 31.0;

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    pub fn draw_background<const BG: u8>(&self) -> egui::ImageData {
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
        let mut pixels: Vec<egui::Color32> = vec![egui::Color32::BLACK; (size.0 * size.1) as usize];
        for map_tile_i in 0..=1023 {
            let map_tile_address = (screen_base + map_tile_i * 2) as usize;
            let mut map_tile_bytes = [0; 2];
            map_tile_bytes.copy_from_slice(&self.bg_vram[map_tile_address..map_tile_address + 2]);
            let map_tile = u16::from_le_bytes(map_tile_bytes);

            let tile_number = map_tile.get_bits(0, 9) as u32;
            let horizonal_flip = map_tile.get_bit(10);
            let vertical_flip = map_tile.get_bit(11);
            let palette_number = map_tile.get_bits(12, 15); // not used in 256/1

            match color_palette {
                ColorPalette::Is16x16 => {
                    // TODO: implement
                }
                ColorPalette::Is256x1 => {
                    let tile_address = (character_base + tile_number * 64) as usize;
                    let tile_bytes = &self.bg_vram[tile_address..tile_address + 64];
                    (0..tile_bytes.len()).for_each(|tile_byte_i| {
                        let tile_byte = tile_bytes[tile_byte_i];
                        let palette_address = tile_byte as usize * 2;
                        let mut map_tile_bytes = [0; 2];
                        map_tile_bytes
                            .copy_from_slice(&self.palette[palette_address..palette_address + 2]);
                        let color = u16::from_le_bytes(map_tile_bytes);
                        let r = ((color.get_bits(0, 4) as f32) * COLOUR_MULT) as u8;
                        let g = ((color.get_bits(5, 9) as f32) * COLOUR_MULT) as u8;
                        let b = ((color.get_bits(10, 14) as f32) * COLOUR_MULT) as u8;

                        // TODO: these will probably all need to be adjusted for different map sizes
                        let tile_byte_i = tile_byte_i as u32;
                        let map_tile_x = map_tile_i % 32;
                        let map_tile_y = map_tile_i / 32;
                        let tile_x = tile_byte_i % 8;
                        let tile_y = tile_byte_i / 8;
                        let pos = (map_tile_x * 8 + tile_x) + (map_tile_y * 8 + tile_y) * size.0;

                        pixels[pos as usize] = egui::Color32::from_rgb(r, g, b);
                    });
                }
            }
        }

        egui::ImageData::from(egui::ColorImage {
            pixels,
            size: [size.0 as usize, size.1 as usize], // TODO: this will need to be cropped based on the position of the backgroun
        })
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
