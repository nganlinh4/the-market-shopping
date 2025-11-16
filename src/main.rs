use eframe::egui;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Item {
    code: String,
    description: String,
    price: u32,
}

struct MyApp {
    items: Vec<Item>,
    search: String,
    selected: HashMap<String, i32>,
    print_output: String,
}

impl MyApp {
    fn new() -> Self {
        let content = std::fs::read_to_string("items.csv").unwrap();
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        let items: Vec<Item> = rdr.deserialize().map(|r| r.unwrap()).collect();
        Self {
            items,
            search: String::new(),
            selected: HashMap::new(),
            print_output: String::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Market Shopping App");

            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search);
            });

            ui.add_space(8.0);

            // Create a side-by-side layout using columns
            ui.columns(2, |columns| {
                // Left column: Available items
                columns[0].vertical(|ui| {
                    ui.heading("Available Items");
                    
                    let query = self.search.to_lowercase();
                    let filtered_items: Vec<&Item> = self.items.iter().filter(|item| {
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
                                    let mut amount = *self.selected.get(&item.code).unwrap_or(&0) as f32;
                                    if ui.add(egui::DragValue::new(&mut amount).speed(1.0).clamp_range(0.0..=1000.0)).changed() {
                                        let amt = amount as i32;
                                        if amt > 0 {
                                            self.selected.insert(item.code.clone(), amt);
                                        } else {
                                            self.selected.remove(&item.code);
                                        }
                                    }
                                });
                            }
                        });
                });

                // Right column: Selected items and controls
                columns[1].vertical(|ui| {
                    ui.heading("Selected Items");
                    
                    if self.selected.is_empty() {
                        ui.label("No items selected");
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
                                            ui.label(format!("{:.0} phần", amount));
                                            ui.add_space(10.0);
                                            ui.label(format!("{} won", item.price * *amount as u32));
                                            
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
                        ui.heading("Total Cost:");
                        ui.heading(format!("{:.0} won", total));
                    });

                    ui.add_space(8.0);

                    // Print functionality
                    if ui.button("Print List").clicked() {
                        let output = self.selected.iter().map(|(code, &amount)| {
                            let item = self.items.iter().find(|i| i.code == *code).unwrap();
                            format!("{} {} ({:.0} phần)", code, item.description, amount)
                        }).collect::<Vec<_>>().join("\n");
                        self.print_output = output;
                    }

                    if !self.print_output.is_empty() {
                        ui.add_space(8.0);
                        ui.label("Printed List:");
                        ui.add(egui::TextEdit::multiline(&mut self.print_output).interactive(false).desired_width(100.0));
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Market Shopping App",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            // Try to load Vietnamese font support
            let font_loaded = try_load_vietnamese_font(&cc.egui_ctx);
            if font_loaded {
                eprintln!("Successfully loaded Vietnamese font support");
            } else {
                eprintln!("Using system fonts with Unicode support for Vietnamese text");
            }
            
            Box::new(MyApp::new())
        }),
    )
}

fn try_load_vietnamese_font(ctx: &egui::Context) -> bool {
    // Try multiple approaches to load a font with Vietnamese support
    
    // Method 1: Try common system font names
    let system_fonts = [
        "C:\\Windows\\Fonts\\arial.ttf",
        "C:\\Windows\\Fonts\\segoeui.ttf",
        "C:\\Windows\\Fonts\\tahoma.ttf",
        "/System/Library/Fonts/Arial.ttf",
        "/System/Library/Fonts/Helvetica.ttc",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    ];
    
    for font_path in &system_fonts {
        if let Ok(font_data) = std::fs::read(font_path) {
            if load_font_into_egui(ctx, &font_data, "SystemFont") {
                eprintln!("Loaded system font: {}", font_path);
                return true;
            }
        }
    }
    
    // Method 2: Try to create a simple Unicode-capable font configuration
    setup_unicode_font_fallback(ctx);
    false
}

fn load_font_into_egui(ctx: &egui::Context, font_data: &[u8], font_name: &str) -> bool {
    let mut font_def = egui::FontDefinitions::empty();
    
    font_def.font_data.insert(
        font_name.to_owned(),
        egui::FontData::from_owned(font_data.to_vec()),
    );
    
    // Set as primary font with Unicode support
    font_def.families.insert(
        egui::FontFamily::default(),
        vec![font_name.to_owned()],
    );
    
    // Also set for monospace
    font_def.families.insert(
        egui::FontFamily::Monospace,
        vec![font_name.to_owned()],
    );
    
    ctx.set_fonts(font_def);
    true
}

fn setup_unicode_font_fallback(ctx: &egui::Context) {
    let mut font_def = egui::FontDefinitions::empty();
    
    // Configure font families with proper Unicode fallbacks
    font_def.families.insert(
        egui::FontFamily::default(),
        vec![
            "Segoe UI".to_owned(),
            "Arial".to_owned(), 
            "DejaVu Sans".to_owned(),
            "Liberation Sans".to_owned(),
            "sans-serif".to_owned(),
        ],
    );
    
    font_def.families.insert(
        egui::FontFamily::Monospace,
        vec![
            "Consolas".to_owned(),
            "Courier New".to_owned(),
            "monospace".to_owned(),
        ],
    );
    
    ctx.set_fonts(font_def);
    eprintln!("Configured Unicode font fallback chain");
}
