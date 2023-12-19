use crate::ui::{NitrousGUI, NitrousWindow};

impl NitrousGUI {
    pub fn show_memory_viewer(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("Memory Viewer", ctx)
            .open(&mut self.memory_viewer)
            .show(ctx, |ui| {
                let mem = &mut self.emulator.mem;
                let text_style = egui::TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                let total_rows = mem.len() / 16;
                egui::ScrollArea::vertical().show_rows(
                    ui,
                    row_height,
                    total_rows,
                    |ui, row_range| {
                        for row in row_range {
                            let row_start = row * 16;
                            let row_end = row_start + 16;
                            let row_mem = &mem[row_start..row_end];

                            ui.horizontal(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("{:08X}", row_start))
                                        .font(egui::FontId::monospace(row_height))
                                        .strong(),
                                );

                                ui.add_space(8.0);
                                for (i, b) in row_mem.iter().enumerate() {
                                    let i = row_start + i;
                                    let selected = self.memory_viewer_selected == Some(i);

                                    let mut is_editing = false;
                                    let byte = if selected
                                        && self.memory_viewer_selected_pending_value.is_some()
                                    {
                                        is_editing = true;
                                        format!(
                                            "{:01X}.",
                                            self.memory_viewer_selected_pending_value.unwrap()
                                        )
                                    } else {
                                        format!("{:02X}", *b)
                                    };

                                    let mut button = egui::Button::new(
                                        egui::RichText::new(byte)
                                            .font(egui::FontId::monospace(row_height)),
                                    )
                                    .frame(false)
                                    .selected(selected);

                                    if is_editing {
                                        // have to reset the frame lol
                                        button = button.fill(egui::Color32::DARK_RED).frame(false);
                                    }

                                    if ui.add(button).clicked() {
                                        if selected {
                                            self.memory_viewer_selected = None;
                                        } else {
                                            self.memory_viewer_selected = Some(i);
                                        }
                                        self.memory_viewer_selected_pending_value = None;
                                    }
                                }

                                ui.add_space(8.0);
                                ui.spacing_mut().item_spacing.x = 0.0;
                                for (i, b) in row_mem.iter().enumerate() {
                                    let i = row_start + i;
                                    let selected = self.memory_viewer_selected == Some(i);

                                    let c = *b as char;
                                    let char = if c.is_ascii_graphic() {
                                        format!("{}", c)
                                    } else {
                                        ".".to_string()
                                    };

                                    let button = egui::Button::new(
                                        egui::RichText::new(char)
                                            .font(egui::FontId::monospace(row_height)),
                                    )
                                    .frame(false)
                                    .selected(selected);

                                    if ui.add(button).clicked() {
                                        if selected {
                                            self.memory_viewer_selected = None;
                                        } else {
                                            self.memory_viewer_selected = Some(i);
                                        }
                                        self.memory_viewer_selected_pending_value = None;
                                    }
                                }
                            });
                        }
                    },
                );

                ui.input(|i| {
                    i.events.iter().for_each(|e| {
                        let selected = if let Some(selected) = self.memory_viewer_selected {
                            selected
                        } else {
                            return;
                        };

                        match e {
                            egui::Event::Key {
                                key: egui::Key::ArrowUp,
                                pressed: true,
                                ..
                            } => {
                                if selected >= 16 {
                                    self.memory_viewer_selected = Some(selected - 16);
                                }
                            }
                            egui::Event::Key {
                                key: egui::Key::ArrowDown,
                                pressed: true,
                                ..
                            } => {
                                if selected < mem.len() - 16 {
                                    self.memory_viewer_selected = Some(selected + 16);
                                }
                            }
                            egui::Event::Key {
                                key: egui::Key::ArrowLeft,
                                pressed: true,
                                ..
                            } => {
                                if selected > 0 {
                                    self.memory_viewer_selected = Some(selected - 1);
                                }
                            }
                            egui::Event::Key {
                                key: egui::Key::ArrowRight,
                                pressed: true,
                                ..
                            } => {
                                if selected < mem.len() - 1 {
                                    self.memory_viewer_selected = Some(selected + 1);
                                }
                            }
                            // egui::Event::Key {
                            //     key,
                            //     pressed,
                            //     repeat,
                            //     modifiers,
                            // } => {
                            //     if *pressed && !*repeat {
                            //         if let Some(selected) = self.memory_viewer_selected {
                            //             if let Some(c) = key.to_char(*modifiers) {
                            //                 let mut b = mem[selected];
                            //                 b = match c {
                            //                     '0'..='9' => {
                            //                         let n = c.to_digit(10).unwrap() as u8;
                            //                         (b & 0xF0) | n
                            //                     }
                            //                     'a'..='f' => {
                            //                         let n = c as u8 - b'a' + 10;
                            //                         (b & 0xF0) | n
                            //                     }
                            //                     'A'..='F' => {
                            //                         let n = c as u8 - b'A' + 10;
                            //                         (b & 0xF0) | n
                            //                     }
                            //                     _ => b,
                            //                 };
                            //                 mem[selected] = b;
                            //             }
                            //         }
                            //     }
                            // }
                            egui::Event::Text(t) => {
                                let b = match t.chars().next().unwrap() {
                                    '0'..='9' => {
                                        Some(t.chars().next().unwrap().to_digit(10).unwrap() as u8)
                                    }
                                    'a'..='f' => Some(t.chars().next().unwrap() as u8 - b'a' + 10),
                                    'A'..='F' => Some(t.chars().next().unwrap() as u8 - b'A' + 10),
                                    _ => None,
                                };

                                if let Some(b) = b {
                                    if let Some(value) = self.memory_viewer_selected_pending_value {
                                        let b = (value << 4) | b;
                                        self.memory_viewer_selected_pending_value = None;
                                        mem[selected] = b;

                                        if selected < mem.len() - 1 {
                                            self.memory_viewer_selected = Some(selected + 1);
                                        }
                                    } else {
                                        self.memory_viewer_selected_pending_value = Some(b);
                                    }
                                }
                            }
                            _ => {}
                        }
                    });
                })
            });
    }
}
