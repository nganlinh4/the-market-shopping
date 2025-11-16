use eframe::egui;
use crate::app::MyApp;
use crate::i18n::Translations;
use crate::models::Language;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let trans = Translations::new(self.language);

            // All header controls in one row
            ui.horizontal(|ui| {
                // Language selector
                ui.label("Language:");
                if ui.selectable_label(self.language == Language::English, trans.english()).clicked() {
                    self.language = Language::English;
                }
                if ui.selectable_label(self.language == Language::Vietnamese, trans.vietnamese()).clicked() {
                    self.language = Language::Vietnamese;
                }
                
                ui.separator();
                
                // Title
                ui.heading(trans.heading());
                
                ui.separator();
                
                // Search
                ui.label(trans.search());
                ui.text_edit_singleline(&mut self.search);
                
                ui.separator();
                
                // Edit raw list button
                if ui.button(trans.edit_list()).clicked() {
                    self.show_raw_editor = true;
                }
            });

            ui.add_space(8.0);

            // Create a side-by-side layout using columns
            ui.columns(2, |columns| {
                // Left column: Available items
                columns[0].vertical(|ui| {
                    ui.heading(trans.available_items());

                    let query = self.search.to_lowercase();
                    let filtered_items: Vec<&crate::models::Item> = self.items.iter().filter(|item| {
                        query.is_empty()
                            || item.code.to_lowercase().contains(&query)
                            || item.description.to_lowercase().contains(&query)
                    }).collect();

                    egui::ScrollArea::vertical()
                        .id_source("items_scroll")
                        .min_scrolled_height(500.0)
                        .show(ui, |ui| {
                            for item in &filtered_items {
                                ui.horizontal(|ui| {
                                    ui.label(format!("{}: {} - {} won", item.code, item.description, item.price));

                                    let amount = *self.selected.get(&item.code).unwrap_or(&0);

                                    if ui.button("-").clicked() {
                                        let new_amt = amount - 1;
                                        if new_amt > 0 {
                                            *self.selected.get_mut(&item.code).unwrap_or(&mut 0) = new_amt;
                                        } else {
                                            self.selected.remove(&item.code);
                                        }
                                    }
                                    ui.label(amount.to_string());
                                    if ui.button("+").clicked() {
                                        self.selected.insert(item.code.clone(), amount + 1);
                                    }
                                });
                            }
                        });
                });

                // Right column: Selected items and controls
                columns[1].vertical(|ui| {
                    ui.heading(trans.selected_items());

                    if self.selected.is_empty() {
                        ui.label(trans.no_items_selected());
                    } else {
                        egui::ScrollArea::vertical()
                            .id_source("selected_scroll")
                            .min_scrolled_height(300.0)
                            .show(ui, |ui| {
                                for (code, amount) in &self.selected.clone() {
                                    if let Some(item) = self.items.iter().find(|i| &i.code == code) {
                                        ui.horizontal(|ui| {
                                            ui.label(format!("{} {}", code, item.description));
                                            ui.add_space(10.0);
                                            ui.label(format!("{} {}", amount, trans.phan()));
                                            ui.add_space(10.0);
                                            ui.label(format!("{} {}", item.price * *amount as u32, trans.won()));

                                            if ui.button("-").clicked() {
                                                let new_amt = *amount - 1;
                                                if new_amt > 0 {
                                                    *self.selected.get_mut(code).unwrap() = new_amt;
                                                } else {
                                                    self.selected.remove(code);
                                                }
                                            }
                                            ui.label(amount.to_string());
                                            if ui.button("+").clicked() {
                                                *self.selected.get_mut(code).unwrap() += 1;
                                            }
                                        });
                                    }
                                }
                            });
                    }

                    ui.add_space(16.0);

                    // Total cost calculation
                    let total: f32 = self.selected.iter().map(|(code, &amount)| {
                        self.items.iter().find(|i| i.code == *code).unwrap().price as f32 * amount as f32
                    }).sum();

                    // Total cost section
                    ui.horizontal(|ui| {
                        ui.heading(trans.total_cost());
                        ui.heading(format!("{:.0} {}", total, trans.won()));
                    });

                    ui.add_space(8.0);

                    // Print functionality
                    if ui.button(trans.print_list()).clicked() {
                        let output = self.selected.iter().map(|(code, &amount)| {
                            let item = self.items.iter().find(|i| i.code == *code).unwrap();
                            format!("{} {} ({} {})", code, item.description, amount, trans.phan())
                        }).collect::<Vec<_>>().join("\n");
                        self.print_output = output;
                    }

                    if !self.print_output.is_empty() {
                        ui.add_space(8.0);
                        ui.label(trans.printed_list());
                        ui.add(egui::TextEdit::multiline(&mut self.print_output).interactive(false).desired_width(f32::INFINITY).desired_rows(5));
                        if ui.button(trans.copy_to_clipboard()).clicked() {
                            ui.ctx().copy_text(self.print_output.clone());
                        }
                    }
                });
            });

            // Raw list editor window
            if self.show_raw_editor {
                let window_title = trans.raw_list_editor().to_string();
                let save_text = trans.save().to_string();
                let cancel_text = trans.cancel().to_string();

                egui::Window::new(window_title)
                    .default_width(700.0)
                    .default_height(500.0)
                    .show(ctx, |ui| {
                        // Buttons at top
                        ui.horizontal(|ui| {
                            if ui.button(&save_text).clicked() {
                                if let Err(e) = std::fs::write("raw-list.txt", &self.raw_content) {
                                    eprintln!("Failed to save raw-list.txt: {}", e);
                                } else {
                                    // Re-parse items from raw-list.txt
                                    self.items = crate::app::MyApp::parse_raw_list(&self.raw_content);
                                    self.search.clear();
                                    self.show_raw_editor = false;
                                    eprintln!("Successfully saved and re-parsed raw-list.txt");
                                }
                            }
                            if ui.button(&cancel_text).clicked() {
                                // Reload the original content from file
                                if let Ok(content) = std::fs::read_to_string("raw-list.txt") {
                                    self.raw_content = content;
                                }
                                self.show_raw_editor = false;
                            }
                        });

                        ui.separator();

                        // Scrollable text editor
                        egui::ScrollArea::vertical()
                            .auto_shrink([false; 2])
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.raw_content)
                                        .desired_rows(20)
                                        .desired_width(f32::INFINITY),
                                );
                            });
                    });
            }
        });
    }
}
