use eframe::egui;
use crate::app::MyApp;
use crate::i18n::Translations;
use crate::models::{Language, ItemCategory, Theme, ThemeColors};

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let trans = Translations::new(self.language);
        let colors = self.theme.colors();

        // Update notifications
        if let Some((_, time_left)) = &mut self.notification {
            *time_left -= ctx.input(|i| i.unstable_dt);
            if *time_left <= 0.0 {
                self.notification = None;
            }
        }

        // Set background color based on theme
        ctx.set_visuals(egui::Visuals {
            panel_fill: colors.background,
            window_fill: colors.card,
            ..Default::default()
        });

        // Header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.set_height(60.0);
            ui.horizontal(|ui| {
                ui.heading("📊 Market Master");
                ui.separator();
                
                // Global search
                ui.label("🔍");
                ui.text_edit_singleline(&mut self.search);
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Settings dropdown menu
                    ui.menu_button("⚙ Settings", |ui| {
                        if ui.button("📋 Item Manager").clicked() {
                            self.show_item_manager = true;
                            ui.close_menu();
                        }
                        if ui.button("📝 Edit Raw List").clicked() {
                            self.show_raw_editor = true;
                            ui.close_menu();
                        }
                        ui.separator();
                        
                        ui.label("🎨 Theme:");
                        for theme in Theme::all().iter() {
                            if ui.selectable_label(self.theme == *theme, theme.as_str()).clicked() {
                                self.theme = *theme;
                                ui.close_menu();
                            }
                        }
                    });
                    
                    // Language selector
                    if ui.selectable_label(self.language == Language::English, "EN").clicked() {
                        self.language = Language::English;
                    }
                    if ui.selectable_label(self.language == Language::Vietnamese, "VI").clicked() {
                        self.language = Language::Vietnamese;
                    }
                });
            });
        });

        // Main content with three panels
        egui::SidePanel::left("category_panel")
            .resizable(false)
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Categories");
                ui.separator();
                
                // "All Items" button
                if ui.selectable_label(
                    self.selected_category == ItemCategory::Other && self.search.is_empty(),
                    trans.all_items()
                ).clicked() {
                    self.selected_category = ItemCategory::Other;
                    self.search.clear();
                }
                
                // Category buttons
                for category in ItemCategory::all().iter() {
                    if ui.selectable_label(self.selected_category == *category, category.as_str()).clicked() {
                        self.selected_category = *category;
                        self.search.clear();
                    }
                }
            });

        egui::SidePanel::right("cart_panel")
            .resizable(false)
            .default_width(300.0)
            .show(ctx, |ui| {
                shopping_cart_panel(ui, &trans, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            item_browser(ui, &trans, self);
        });

        // Notification toast
        if let Some((message, _)) = &self.notification {
            let colors = self.theme.colors();
            egui::Area::new(egui::Id::new("notification"))
                .anchor(egui::Align2::CENTER_BOTTOM, egui::vec2(0.0, -20.0))
                .show(ctx, |ui| {
                    ui.colored_label(colors.success, format!("✓ {}", message));
                });
        }

        // Item Manager Modal
        if self.show_item_manager {
            item_manager_modal(ctx, &trans, self);
        }

        // Raw editor window (for backward compatibility)
        if self.show_raw_editor {
            raw_editor_window(ctx, &trans, self);
        }
    }
}

fn item_browser(ui: &mut egui::Ui, trans: &Translations, app: &mut MyApp) {
    let colors = app.theme.colors();
    let query = app.search.to_lowercase();
    let selected_category = app.selected_category;
    
    let filtered_items: Vec<crate::models::Item> = app.items
        .clone()
        .into_iter()
        .filter(|item| {
            let matches_search = query.is_empty()
                || item.code.to_lowercase().contains(&query)
                || item.description.to_lowercase().contains(&query);
            
            let matches_category = selected_category == ItemCategory::Other
                || ItemCategory::from_str(&item.category) == selected_category;
            
            matches_search && matches_category
        })
        .collect();

    ui.heading(format!("📦 {} ({} items)", trans.available_items(), filtered_items.len()));
    ui.separator();

    egui::ScrollArea::vertical()
        .id_source("items_scroll")
        .auto_shrink([false; 2])
        .show(ui, |ui| {
            for item in filtered_items {
                item_card(ui, &item, app, trans, colors);
            }
        });
}

fn item_card(ui: &mut egui::Ui, item: &crate::models::Item, app: &mut MyApp, _trans: &Translations, colors: ThemeColors) {
    ui.group(|ui| {
        ui.set_min_height(100.0);
        
        // Header: Code
        ui.colored_label(colors.text_secondary, &item.code);
        
        // Title: Description
        ui.heading(&item.description);
        
        // Bottom row: Price and quantity stepper
        ui.horizontal(|ui| {
            // Price (left)
            ui.colored_label(colors.primary, format!("₩{}", item.price));
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Quantity stepper (right)
                let amount = *app.selected.get(&item.code).unwrap_or(&0);
                
                if amount == 0 {
                    // Show "Add" button when quantity is 0
                    if ui.button("➕ Add").clicked() {
                        app.selected.insert(item.code.clone(), 1);
                    }
                } else {
                    // Show quantity stepper
                    if ui.button("-").clicked() {
                        let new_amt = amount - 1;
                        if new_amt > 0 {
                            app.selected.insert(item.code.clone(), new_amt);
                        } else {
                            app.selected.remove(&item.code);
                        }
                    }
                    
                    // Quantity input field (clickable to edit)
                    let mut qty_str = amount.to_string();
                    let response = ui.text_edit_singleline(&mut qty_str);
                    if response.changed() {
                        if let Ok(new_amt) = qty_str.parse::<i32>() {
                            if new_amt > 0 {
                                app.selected.insert(item.code.clone(), new_amt);
                            } else {
                                app.selected.remove(&item.code);
                            }
                        }
                    }
                    
                    if ui.button("+").clicked() {
                        app.selected.insert(item.code.clone(), amount + 1);
                    }
                }
            });
        });
    });
}

fn shopping_cart_panel(ui: &mut egui::Ui, trans: &Translations, app: &mut MyApp) {
    let colors = app.theme.colors();
    
    ui.heading("🛒 Cart");
    ui.separator();

    if app.selected.is_empty() {
        ui.colored_label(colors.text_secondary, trans.no_items_selected());
    } else {
        egui::ScrollArea::vertical()
            .id_source("cart_scroll")
            .auto_shrink([false; 2])
            .max_height(400.0)
            .show(ui, |ui| {
                for (code, amount) in app.selected.clone().iter() {
                    if let Some(item) = app.items.iter().find(|i| &i.code == code) {
                        let subtotal = item.price as f32 * *amount as f32;
                        
                        ui.horizontal(|ui| {
                            // Item info (left)
                            ui.vertical(|ui| {
                                ui.label(format!("{} {}", &item.code, &item.description));
                                ui.colored_label(colors.text_secondary, format!("{} × ₩{}", amount, item.price));
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Delete button
                                if ui.button("✕").clicked() {
                                    app.selected.remove(code);
                                }
                                
                                // Subtotal
                                ui.colored_label(colors.primary, format!("₩{:.0}", subtotal));
                            });
                        });
                        ui.separator();
                    }
                }
            });
    }

    ui.add_space(16.0);

    // Totals section
    let total: f32 = app.selected.iter().map(|(code, &amount)| {
        app.items.iter().find(|i| i.code == *code).unwrap().price as f32 * amount as f32
    }).sum();

    ui.separator();
    ui.colored_label(colors.primary, format!("Total: ₩{:.0}", total));
    ui.add_space(8.0);

    // Action buttons
    if ui.button(egui::RichText::new(trans.generate_list()).size(16.0)).clicked() {
        let output = app.selected.iter().map(|(code, &amount)| {
            let item = app.items.iter().find(|i| i.code == *code).unwrap();
            format!("{} {} ({} {})", code, item.description, amount, trans.phan())
        }).collect::<Vec<_>>().join("\n");
        app.print_output = output;
    }

    if ui.button(egui::RichText::new(trans.clear_cart()).size(14.0)).clicked() {
        app.selected.clear();
    }

    ui.add_space(8.0);

    // Print output section
    if !app.print_output.is_empty() {
        ui.separator();
        ui.label(trans.printed_list());
        ui.add(
            egui::TextEdit::multiline(&mut app.print_output)
                .interactive(false)
                .desired_width(f32::INFINITY)
                .desired_rows(5)
        );
        
        if ui.button(trans.copy_to_clipboard()).clicked() {
            ui.ctx().copy_text(app.print_output.clone());
            app.notification = Some((trans.copied_notification().to_string(), 2.0));
        }
    }
}

fn item_manager_modal(ctx: &egui::Context, trans: &Translations, app: &mut MyApp) {
    let mut is_open = true;
    
    egui::Window::new(trans.item_manager())
        .open(&mut is_open)
        .default_width(800.0)
        .default_height(600.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(trans.add_item()).clicked() {
                    app.edit_item_code.clear();
                    app.edit_item_description.clear();
                    app.edit_item_price.clear();
                    app.edit_item_category = ItemCategory::Other;
                }
            });
            
            ui.separator();
            
            // Item list table
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    // Header
                    ui.horizontal(|ui| {
                        ui.label("Code");
                        ui.label("Description");
                        ui.label("Price");
                        ui.label("Category");
                        ui.label("Actions");
                    });
                    ui.separator();
                    
                    // Items
                    for item in app.items.clone().iter() {
                        ui.horizontal(|ui| {
                            ui.label(&item.code);
                            ui.label(&item.description);
                            ui.label(format!("₩{}", item.price));
                            ui.label(&item.category);
                            
                            if ui.button("✏").clicked() {
                                app.edit_item_code = item.code.clone();
                                app.edit_item_description = item.description.clone();
                                app.edit_item_price = item.price.to_string();
                                app.edit_item_category = ItemCategory::from_str(&item.category);
                            }
                            
                            if ui.button("🗑").clicked() {
                                app.items.retain(|i| i.code != item.code);
                                let _ = app.save_items_to_csv();
                            }
                        });
                    }
                });
        });
    
    if !is_open {
        app.show_item_manager = false;
    }
}

fn raw_editor_window(ctx: &egui::Context, trans: &Translations, app: &mut MyApp) {
    let mut is_open = true;
    
    egui::Window::new(trans.raw_list_editor())
        .open(&mut is_open)
        .default_width(700.0)
        .default_height(500.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button(trans.save()).clicked() {
                    if let Err(e) = std::fs::write("raw-list.txt", &app.raw_content) {
                        eprintln!("Failed to save raw-list.txt: {}", e);
                    } else {
                        app.items = crate::app::MyApp::parse_raw_list(&app.raw_content);
                        app.search.clear();
                        app.show_raw_editor = false;
                    }
                }
                if ui.button(trans.cancel()).clicked() {
                    if let Ok(content) = std::fs::read_to_string("raw-list.txt") {
                        app.raw_content = content;
                    }
                    app.show_raw_editor = false;
                }
            });
            
            ui.separator();
            
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut app.raw_content)
                            .desired_rows(20)
                            .desired_width(f32::INFINITY)
                    );
                });
        });
    
    if !is_open {
        app.show_raw_editor = false;
    }
}
