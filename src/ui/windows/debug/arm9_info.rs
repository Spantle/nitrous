use crate::nds::cpu::arm;
use crate::ui::{NitrousGUI, NitrousUI, NitrousWindow};

// this code is absolutely horrendous and i'm sorry (i just wanna write the emulator not this important useful crap i'm gonna be using 24/7 lol)
impl NitrousGUI {
    pub fn show_arm9_info(&mut self, ctx: &egui::Context) {
        egui::Window::new_nitrous("ARM9 Info", ctx)
            .open(&mut self.arm9_info)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Register Values (Hexadecimal)")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.make_monospace();

                        let r = self.emulator.arm9.r;
                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .columns(egui_extras::Column::exact(60.0), r.len() + 1)
                            .header(20.0, |mut header| {
                                header.col(|_| ());
                                for i in 0..r.len() {
                                    header.col(|ui| {
                                        match i {
                                            13 => {
                                                ui.strong("r13 (SP)").on_hover_text("Stack Pointer")
                                            }
                                            14 => {
                                                ui.strong("r14 (LR)").on_hover_text("Link Register")
                                            }
                                            15 => ui
                                                .strong("r15 (PC)")
                                                .on_hover_text("Program Counter"),
                                            _ => ui.strong(format!("r{}", i)),
                                        };
                                    });
                                }
                            })
                            .body(|mut body| {
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        ui.strong("Base");
                                    });
                                    for i in 0..r.len() {
                                        let ii = i as u8;
                                        row.col(|ui| {
                                            let mut flag = true;
                                            if let Some(selected) = &self.arm9_info_selected {
                                                if selected.0 == "Base" && selected.1 == i {
                                                    let text_edit = egui::TextEdit::singleline(&mut self.arm9_info_selected_pending_value).char_limit(8).frame(false).margin(egui::Vec2::new(0.0, 0.0));
                                                    if ui.add(text_edit).lost_focus() {
                                                        let value = u32::from_str_radix(&self.arm9_info_selected_pending_value, 16);
                                                        if let Ok(value) = value {
                                                            self.emulator.arm9.r[ii] = value;
                                                        }
                                                        self.arm9_info_selected = None;
                                                    } else {
                                                        flag = false;
                                                    }
                                                }
                                            }

                                            let value = format!("{:08X}", self.emulator.arm9.r[ii]);
                                            if flag && ui.add(egui::Label::new(&value).sense(egui::Sense::click())).clicked()
                                            {
                                                self.arm9_info_selected_pending_value = value;
                                                self.arm9_info_selected = Some(("Base".to_string(), i));
                                            };
                                        });
                                    }
                                });

                                let mut display_range =
                                    |name: &str, min: usize, max: usize, registers: &mut [u32]| {
                                        body.row(20.0, |mut row| {
                                            row.col(|ui| {
                                                ui.strong(name);
                                            });
                                            for i in 0..r.len() {
                                                if i < min || i > max {
                                                    row.col(|_ui| {});
                                                    continue;
                                                }


                                                // cargo fmt has given up and so have i
                                                row.col(|ui| {
                                                    let mut flag = true;
                                                    if let Some(selected) = &self.arm9_info_selected {
                                                        if selected.0 == *name && selected.1 == i - min {
                                                            let text_edit = egui::TextEdit::singleline(&mut self.arm9_info_selected_pending_value).char_limit(8).frame(false).margin(egui::Vec2::new(0.0, 0.0));
                                                            if ui.add(text_edit).lost_focus() {
                                                                let value = u32::from_str_radix(&self.arm9_info_selected_pending_value, 16);
                                                                if let Ok(value) = value {
                                                                    registers[i - min] = value;
                                                                }
                                                                self.arm9_info_selected = None;
                                                            } else {
                                                                flag = false;
                                                            }
                                                        }
                                                    }

                                                    let value = format!("{:08X}", registers[i - min]);
                                                    if flag && ui.add(egui::Label::new(&value).sense(egui::Sense::click())).clicked()
                                                    {
                                                        self.arm9_info_selected_pending_value = value;
                                                        self.arm9_info_selected = Some((name.to_string(), i - min));
                                                    };
                                                });
                                            }
                                        });
                                    };

                                display_range("FIQ", 8, 14, &mut self.emulator.arm9.r_fiq);
                                display_range("IRQ", 13, 14, &mut self.emulator.arm9.r_irq);
                                display_range("SVC", 13, 14, &mut self.emulator.arm9.r_svc);
                                display_range("ABT", 13, 14, &mut self.emulator.arm9.r_abt);
                                display_range("UND", 13, 14, &mut self.emulator.arm9.r_und);
                            });
                    });

                egui::CollapsingHeader::new("Program Status Registers")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.make_monospace();

                        egui_extras::TableBuilder::new(ui)
                            .striped(true)
                            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                            .column(egui_extras::Column::exact(60.0))
                            .column(egui_extras::Column::exact(128.0))
                            .columns(egui_extras::Column::exact(60.0), 8)
                            .column(egui_extras::Column::exact(256.0))
                            .header(20.0, |mut header| {
                                header.col(|_| ());
                                header.col(|ui| {
                                    ui.strong("Mode");
                                });
                                header.col(|ui| {
                                    ui.strong("T").on_hover_text("Thumb");
                                });
                                header.col(|ui| {
                                    ui.strong("F").on_hover_text("FIQ Interrupt");
                                });
                                header.col(|ui| {
                                    ui.strong("I").on_hover_text("IRQ Interrupt");
                                });
                                header.col(|ui| {
                                    ui.strong("Q").on_hover_text("Saturation");
                                });
                                header.col(|ui| {
                                    ui.strong("V").on_hover_text("Overflow");
                                });
                                header.col(|ui| {
                                    ui.strong("C").on_hover_text("Carry");
                                });
                                header.col(|ui| {
                                    ui.strong("Z").on_hover_text("Zero");
                                });
                                header.col(|ui| {
                                    ui.strong("N").on_hover_text("Negative");
                                });
                                header.col(|ui| {
                                    ui.strong("Binary Value");
                                });
                            })
                            .body(|mut body| {
                                let mut display_psr = |name: &str, psr: &arm::models::PSR| {
                                    body.row(20.0, |mut row| {
                                        let flag = |ui: &mut egui::Ui, flag: bool| {
                                            let v = format!("{}", flag);
                                            if flag {
                                                ui.label(egui::RichText::new(v).color(ui.visuals().widgets.inactive.text_color()))
                                            } else {
                                                ui.label(egui::RichText::new(v))
                                            }
                                        };
                                        row.col(|ui| {
                                            ui.strong(name);
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{:?}", psr.get_mode()));
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_thumb());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_fiq_interrupt());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_irq_interrupt());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_saturation());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_overflow());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_carry());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_zero());
                                        });
                                        row.col(|ui| {
                                            flag(ui, psr.get_negative());
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{:032b}", psr.value()));
                                        });
                                    });
                                };

                                display_psr("Base", &self.emulator.arm9.cpsr);
                                display_psr("FIQ", &self.emulator.arm9.r_fiq[7].into());
                                display_psr("IRQ", &self.emulator.arm9.r_irq[2].into());
                                display_psr("SVC", &self.emulator.arm9.r_svc[2].into());
                                display_psr("ABT", &self.emulator.arm9.r_abt[2].into());
                                display_psr("UND", &self.emulator.arm9.r_und[2].into());
                            });
                    });
            });
    }
}
