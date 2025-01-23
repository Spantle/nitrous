use crate::nds::{
    gpus::{
        gpu2d::{BackgroundResult, Gpu2d},
        vram::VramBanks,
    },
    Bits, Bytes,
};

impl<const ENGINE_A: bool> Gpu2d<ENGINE_A> {
    pub fn render_objs(&self, vram_banks: &VramBanks) -> BackgroundResult {
        let mut pixels: Vec<Vec<u16>> = vec![vec![0; 255]; 511];
        let obj_vram_base: usize = if ENGINE_A { 0x06400000 } else { 0x06600000 };

        for i in 0..128 {
            let addr = i * 8;

            let mut oam0 = [0; 2];
            let mut oam1 = [0; 2];
            let mut oam2 = [0; 2];
            oam0.copy_from_slice(&self.oam[addr..=(addr + 1)]);
            oam1.copy_from_slice(&self.oam[addr + 2..=(addr + 2 + 1)]);
            oam2.copy_from_slice(&self.oam[addr + 4..=(addr + 4 + 1)]);
            let oam0 = oam0.into_halfword();
            let oam1 = oam1.into_halfword();
            let oam2 = oam2.into_halfword();

            if oam0.get_bit(9) {
                continue;
            }

            let character_name = oam2.get_bits(0, 9) as usize;
            let palette_number = oam2.get_bits(12, 15);
            let palette_offset_address = palette_number << 4;
            let (width, height) = match (oam0.get_bits(14, 15), oam1.get_bits(14, 15)) {
                // shape, size
                (0, 0) => (8, 8),
                (0, 1) => (16, 16),
                (0, 2) => (32, 32),
                (0, 3) => (64, 64),
                (1, 0) => (16, 8),
                (1, 1) => (32, 8),
                (1, 2) => (32, 16),
                (1, 3) => (64, 32),
                (2, 0) => (8, 16),
                (2, 1) => (8, 32),
                (2, 2) => (16, 32),
                (2, 3) => (32, 64),
                (3, 0..=3) => continue,
                _ => unreachable!(),
            };

            let x = oam1.get_bits(0, 8) as usize;
            let y = oam0.get_bits(0, 7) as usize;
            for obj_quad_y in 0..height / 8 {
                for obj_quad_x in 0..width / 8 {
                    // let obj_bytes = vram_banks.read_slice::<64>(
                    //     obj_vram_base
                    //         + (character_name as usize + obj_quad_x + (obj_quad_y * width))
                    //             * 8
                    //             * 8
                    //             * 2,
                    // );
                    let obj_offset =
                        (character_name * 4 + obj_quad_x + (obj_quad_y * (width / 8))) * 32;
                    let obj_bytes = vram_banks.read_slice::<32>(obj_vram_base + obj_offset);
                    let obj_bytes = obj_bytes.unwrap_or([0; 32]);
                    (0..32).for_each(|obj_byte_i| {
                        if false {
                            let obj_byte = obj_bytes[obj_byte_i];
                            let palette_address =
                                (512 + obj_byte as usize * 2) | palette_offset_address as usize;
                            let mut obj_bytes = [0; 2];
                            obj_bytes.copy_from_slice(
                                &self.palette[palette_address..palette_address + 2],
                            );
                            let mut color = u16::from_le_bytes(obj_bytes);
                            color.set_bit(15, true);

                            let obj_pixel_x = obj_byte_i % 8;
                            let obj_pixel_y = obj_byte_i / 8;
                            let pixel_x = x + (obj_quad_x * 8) + obj_pixel_x;
                            let pixel_y = y + (obj_quad_y * 8) + obj_pixel_y;

                            // let mut color = 0b1_00000_00000_00000;
                            // let priority = oam2.get_bits(10, 11);
                            // match priority {
                            //     0 => color.set_bits(0, 4, 0b11111),
                            //     1 => color.set_bits(5, 9, 0b11111),
                            //     2 => color.set_bits(10, 14, 0b11111),
                            //     3 => color.set_bits(0, 9, 0b1111111111),
                            //     _ => unreachable!(),
                            // }

                            pixels[pixel_x % 511][pixel_y % 255] = color;
                        } else {
                            let obj_byte = obj_bytes[obj_byte_i];
                            let l_obj_palette_i = obj_byte.get_bits(0, 3) as u32;
                            let r_obj_palette_i = obj_byte.get_bits(4, 7) as u32;
                            let l_palette_address = (512 + l_obj_palette_i as usize * 2)
                                | palette_offset_address as usize;
                            let r_palette_address = (512 + r_obj_palette_i as usize * 2)
                                | palette_offset_address as usize;

                            let mut l_obj_bytes = [0; 2];
                            let mut r_obj_bytes = [0; 2];
                            l_obj_bytes.copy_from_slice(
                                &self.palette[l_palette_address..l_palette_address + 2],
                            );
                            r_obj_bytes.copy_from_slice(
                                &self.palette[r_palette_address..r_palette_address + 2],
                            );
                            let mut l_color = u16::from_le_bytes(l_obj_bytes);
                            let mut r_color = u16::from_le_bytes(r_obj_bytes);
                            l_color.set_bit(15, true);
                            r_color.set_bit(15, true);

                            let l_obj_pixel_x = (obj_byte_i * 2) % 8;
                            let r_obj_pixel_x = ((obj_byte_i * 2) + 1) % 8;
                            let obj_pixel_y = obj_byte_i * 2 / 8;

                            let l_pixel_x = x + (obj_quad_x * 8) + l_obj_pixel_x;
                            let r_pixel_x = x + (obj_quad_x * 8) + r_obj_pixel_x;
                            let pixel_y = y + (obj_quad_y * 8) + obj_pixel_y;

                            // let mut color = 0b1_00000_00000_00000;
                            // let priority = oam2.get_bits(10, 11);
                            // match priority {
                            //     0 => color.set_bits(0, 4, 0b11111),
                            //     1 => color.set_bits(5, 9, 0b11111),
                            //     2 => color.set_bits(10, 14, 0b11111),
                            //     3 => color.set_bits(0, 9, 0b1111111111),
                            //     _ => unreachable!(),
                            // }

                            pixels[l_pixel_x % 511][pixel_y % 255] = l_color;
                            pixels[r_pixel_x % 511][pixel_y % 255] = r_color;
                        }
                    });
                }
            }
        }

        (pixels, true)
    }
}
