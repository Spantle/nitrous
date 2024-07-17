use crate::ui::{NitrousGUI, NitrousUI, NitrousWindow};

impl NitrousGUI {
    pub fn show_memory_viewer(&mut self, ctx: &egui::Context) {
        let window = egui::Window::new_nitrous("Memory Viewer", ctx)
            .open(&mut self.memory_viewer)
            .show(ctx, |ui| {
                let visual_offset = 0x02000000;
                let mem = &mut self.emulator.shared.psram;
                let text_style = egui::TextStyle::Monospace;
                let height = ui.text_style_height(&text_style);
                let total_rows = mem.len() / 16;
                egui::ScrollArea::vertical().show_rows(ui, height, total_rows, |ui, row_range| {
                    for row in row_range {
                        let row_start = row * 16;
                        let row_end = row_start + 16;
                        let row_mem = &mem[row_start..row_end];

                        ui.horizontal(|ui| {
                            ui.make_monospace();

                            ui.strong(format!("{:08X}", row_start + visual_offset));

                            ui.add_space(height / 2.0);
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

                                let mut button =
                                    egui::Button::new(byte).frame(false).selected(selected);

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

                            ui.add_space(height / 2.0);
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

                                let button =
                                    egui::Button::new(char).frame(false).selected(selected);

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
                });

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
                            egui::Event::Key {
                                key: egui::Key::Backspace,
                                pressed: true,
                                ..
                            }
                            | egui::Event::Key {
                                key: egui::Key::Delete,
                                pressed: true,
                                ..
                            }
                            | egui::Event::Key {
                                key: egui::Key::Escape,
                                pressed: true,
                                ..
                            } => {
                                if self.memory_viewer_selected_pending_value.is_some() {
                                    self.memory_viewer_selected_pending_value = None;
                                }
                            }
                            egui::Event::Paste(contents) => {
                                let mut chars = contents.chars();
                                let mut i = selected;
                                while let Some(char) = chars.next() {
                                    let char = validate_char(char);
                                    match char {
                                        ValidateCharResult::Valid(char) => {
                                            if let Some(char2) = chars.next() {
                                                let char2 = validate_char(char2);
                                                match char2 {
                                                    ValidateCharResult::Valid(char2) => {
                                                        let b = (char << 4) | char2;
                                                        mem[i] = b;
                                                        i += 1;
                                                        continue;
                                                    }
                                                    ValidateCharResult::Invalid => {}
                                                    ValidateCharResult::Skip => continue,
                                                }
                                            }

                                            self.memory_viewer_selected_pending_value = Some(char);
                                        }
                                        ValidateCharResult::Invalid => break,
                                        ValidateCharResult::Skip => continue,
                                    };

                                    break;
                                }

                                self.memory_viewer_selected = Some(i);
                            }
                            egui::Event::Text(text) => {
                                let char = validate_char(text.chars().next().unwrap());

                                if let ValidateCharResult::Valid(char) = char {
                                    if let Some(value) = self.memory_viewer_selected_pending_value {
                                        let b = (value << 4) | char;
                                        self.memory_viewer_selected_pending_value = None;
                                        mem[selected] = b;

                                        if selected < mem.len() - 1 {
                                            self.memory_viewer_selected = Some(selected + 1);
                                        }
                                    } else {
                                        self.memory_viewer_selected_pending_value = Some(char);
                                    }
                                }
                            }
                            _ => {}
                        }
                    });
                })
            });

        if let Some(window) = window {
            // for some reason the focus methods aren't working
            // thanks egui
            if window.response.clicked_elsewhere() {
                self.memory_viewer_selected = None;
                self.memory_viewer_selected_pending_value = None;
            }
        }
    }
}

fn validate_char(char: char) -> ValidateCharResult {
    match char {
        '0'..='9' => ValidateCharResult::Valid(char.to_digit(10).unwrap() as u8),
        'a'..='f' => ValidateCharResult::Valid(char as u8 - b'a' + 10),
        'A'..='F' => ValidateCharResult::Valid(char as u8 - b'A' + 10),
        ' ' | '\t' | '\n' | '\r' => ValidateCharResult::Skip,
        _ => ValidateCharResult::Invalid,
    }
}

enum ValidateCharResult {
    Valid(u8),
    Invalid,
    Skip,
}
