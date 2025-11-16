use crate::models::{Item, Language, ItemCategory, Theme};
use std::collections::HashMap;

pub struct MyApp {
    pub items: Vec<Item>,
    pub search: String,
    pub selected: HashMap<String, i32>,
    pub print_output: String,
    pub language: Language,
    pub theme: Theme,
    pub show_raw_editor: bool,
    pub raw_content: String,
    pub selected_category: ItemCategory,
    pub show_item_manager: bool,
    pub edit_item_code: String,
    pub edit_item_description: String,
    pub edit_item_price: String,
    pub edit_item_category: ItemCategory,
    pub notification: Option<(String, f32)>, // message, remaining_time
}

impl MyApp {
    pub fn new() -> Self {
        let csv_content = std::fs::read_to_string("items.csv").unwrap();
        let raw_content = std::fs::read_to_string("raw-list.txt").unwrap();
        let items = Self::parse_items(&csv_content);
        Self {
            items,
            search: String::new(),
            selected: HashMap::new(),
            print_output: String::new(),
            language: Language::Vietnamese,
            theme: Theme::Light,
            show_raw_editor: false,
            raw_content,
            selected_category: ItemCategory::Other,
            show_item_manager: false,
            edit_item_code: String::new(),
            edit_item_description: String::new(),
            edit_item_price: String::new(),
            edit_item_category: ItemCategory::Other,
            notification: None,
        }
    }

    pub fn parse_items(content: &str) -> Vec<Item> {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        rdr.deserialize().map(|r| r.unwrap()).collect()
    }

    pub fn parse_raw_list(content: &str) -> Vec<Item> {
        let mut items = Vec::new();
        let mut current_category = ItemCategory::Other;
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and info lines
            if line.is_empty() 
                || line.starts_with("🚛")
                || line.starts_with("🎁")
                || line.starts_with("[")
                || line.starts_with("💌")
                || line.starts_with("⭐")
                || line.starts_with("Khi")
                || line.starts_with("Nếu")
                || line.starts_with("RED")
                || line.starts_with("GREEN")
                || line.starts_with("Phụ")
            {
                continue;
            }
            
            // Check for category headers
            if line.starts_with("🍴") {
                continue; // Skip section header
            } else if line.starts_with("🐖") {
                current_category = ItemCategory::Pork;
                continue;
            } else if line.starts_with("🐓") {
                current_category = ItemCategory::Chicken;
                continue;
            } else if line.starts_with("🦆") {
                current_category = ItemCategory::Duck;
                continue;
            } else if line.starts_with("🐂") {
                current_category = ItemCategory::Beef;
                continue;
            } else if line.starts_with("🐟") {
                current_category = ItemCategory::Seafood;
                continue;
            } else if line.starts_with("🍐") {
                current_category = ItemCategory::Vegetables;
                continue;
            } else if line.starts_with("🥫") {
                current_category = ItemCategory::Sauces;
                continue;
            }
            
            // Try to parse item lines like: "CK01 đùi gà 1kg / 6,500 → 5,900"
            if let Some(slash_pos) = line.find('/') {
                let before_slash = line[..slash_pos].trim();
                let after_slash = line[slash_pos + 1..].trim();
                
                // Extract price (handle arrow notation for sales)
                let price_str = if let Some(arrow_pos) = after_slash.find('→') {
                    after_slash[arrow_pos + 1..].trim()
                } else {
                    after_slash
                };
                
                let price_str = price_str
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .replace(",", "");
                
                if let Ok(price) = price_str.parse::<u32>() {
                    let parts: Vec<&str> = before_slash.split_whitespace().collect();
                    if !parts.is_empty() && parts[0].chars().next().map(|c| c.is_ascii_alphanumeric()).unwrap_or(false) {
                        let code = parts[0].to_string();
                        let description = parts[1..].join(" ");
                        
                        if price > 0 && !description.is_empty() {
                            items.push(Item {
                                code,
                                description,
                                price,
                                category: ItemCategory::as_str(&current_category).to_string(),
                            });
                        }
                    }
                }
            }
        }
        
        items
    }
    
    pub fn save_items_to_csv(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_writer(vec![]);
        for item in &self.items {
            wtr.serialize(item)?;
        }
        let data = String::from_utf8(wtr.into_inner()?)?;
        std::fs::write("items.csv", data)?;
        Ok(())
    }


}
