use crate::{
    nds::{Bits, Emulator},
    ui::NitrousWindow,
};

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct PaletteViewerWindow {
    pub open: bool,
}

// TODO: make this global or something stop copy pasting it everywhere
const COLOUR_MULT: f32 = 255.0 / 31.0;

// TODO: improve or finish this, i threw this together in a few minutes and don't care
impl PaletteViewerWindow {
    pub fn show(&mut self, emulator: &Emulator, ctx: &egui::Context) {
        egui::Window::new_nitrous("Palette Viewer", ctx)
            .open(&mut self.open)
            .show(ctx, |ui| {
                let table = egui_extras::TableBuilder::new(ui)
                    .columns(egui_extras::Column::exact(16.0), 16);

                table.body(|mut body| {
                    let palette_a = &emulator.shared.gpus.a.palette;
                    (0..16).for_each(|palette_i| {
                        body.row(16.0, |mut row| {
                            (0..16).for_each(|color_i| {
                                let palette_address = (palette_i * 16 + color_i) * 2;
                                let mut color_bytes = [0; 2];
                                color_bytes.copy_from_slice(
                                    &palette_a[palette_address..palette_address + 2],
                                );
                                let color = u16::from_le_bytes(color_bytes);

                                let r = ((color.get_bits(0, 4) as f32) * COLOUR_MULT) as u8;
                                let g = ((color.get_bits(5, 9) as f32) * COLOUR_MULT) as u8;
                                let b = ((color.get_bits(10, 14) as f32) * COLOUR_MULT) as u8;
                                let size = egui::Vec2::splat(16.0);
                                row.col(|ui| {
                                    let (rect, _response) =
                                        ui.allocate_at_least(size, egui::Sense::hover());
                                    ui.painter().rect_filled(
                                        rect,
                                        1.0,
                                        egui::Color32::from_rgba_premultiplied(r, g, b, 255),
                                    );
                                });
                            });
                        });
                    });
                });
            });
    }
}
