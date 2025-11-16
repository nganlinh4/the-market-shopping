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
        let mut rdr = csv::Reader::from_path("items.csv").unwrap();
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

            let query = self.search.to_lowercase();
            let filtered_items: Vec<&Item> = self.items.iter().filter(|item| {
                query.is_empty()
                    || item.code.to_lowercase().contains(&query)
                    || item.description.to_lowercase().contains(&query)
            }).collect();

            ui.heading("Items");
            egui::ScrollArea::vertical().show(ui, |ui| {
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

            ui.separator();

            ui.heading("Selected Items");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (code, amount) in &self.selected.clone() {
                    if let Some(item) = self.items.iter().find(|i| &i.code == code) {
                        ui.horizontal(|ui| {
                            ui.label(format!("{} {} ({:.0} phần)", code, item.description, amount));
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

            ui.separator();

            let total: f32 = self.selected.iter().map(|(code, &amount)| {
                self.items.iter().find(|i| i.code == *code).unwrap().price as f32 * amount as f32
            }).sum();
            ui.label(format!("Total Cost: {:.0} won", total));

            if ui.button("Print List").clicked() {
                let output = self.selected.iter().map(|(code, &amount)| {
                    let item = self.items.iter().find(|i| i.code == *code).unwrap();
                    format!("{} {} ({:.0} phần)", code, item.description, amount)
                }).collect::<Vec<_>>().join("\n");
                self.print_output = output;
            }

            if !self.print_output.is_empty() {
                ui.label("Printed List:");
                ui.add(egui::TextEdit::multiline(&mut self.print_output).interactive(false));
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Market Shopping App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}
