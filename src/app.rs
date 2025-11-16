use crate::models::{Item, Language};
use std::collections::HashMap;

pub struct MyApp {
    pub items: Vec<Item>,
    pub search: String,
    pub selected: HashMap<String, i32>,
    pub print_output: String,
    pub language: Language,
    pub show_raw_editor: bool,
    pub raw_content: String,
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
            show_raw_editor: false,
            raw_content,
        }
    }

    pub fn parse_items(content: &str) -> Vec<Item> {
        let mut rdr = csv::Reader::from_reader(content.as_bytes());
        rdr.deserialize().map(|r| r.unwrap()).collect()
    }

    pub fn parse_raw_list(content: &str) -> Vec<Item> {
        let mut items = Vec::new();
        
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines, headers, sections, and info lines
            if line.is_empty() 
                || line.starts_with("🚛")
                || line.starts_with("🎁")
                || line.starts_with("[")
                || line.starts_with("💌")
                || line.starts_with("🍴")
                || line.starts_with("🐖")
                || line.starts_with("🐓")
                || line.starts_with("🦆")
                || line.starts_with("🐂")
                || line.starts_with("🐟")
                || line.starts_with("🍐")
                || line.starts_with("🥫")
                || line.starts_with("⭐")
                || line.starts_with("Khi")
                || line.starts_with("Nếu")
                || line.starts_with("RED")
                || line.starts_with("GREEN")
                || line.starts_with("Phụ")
            {
                continue;
            }
            
            // Try to parse item lines like: "CK01 đùi gà 1kg / 6,500 → 5,900"
            // Extract code (alphanumeric start), description, and price
            if let Some(slash_pos) = line.find('/') {
                let before_slash = line[..slash_pos].trim();
                let after_slash = line[slash_pos + 1..].trim();
                
                // Extract price (the last number before any emoji or text)
                let price_str = after_slash
                    .split_whitespace()
                    .next()
                    .unwrap_or("0")
                    .replace(",", "");
                
                if let Ok(price) = price_str.parse::<u32>() {
                    // Extract code (first word)
                    let parts: Vec<&str> = before_slash.split_whitespace().collect();
                    if !parts.is_empty() && parts[0].chars().next().map(|c| c.is_ascii_alphanumeric()).unwrap_or(false) {
                        let code = parts[0].to_string();
                        let description = parts[1..].join(" ");
                        
                        if price > 0 && !description.is_empty() {
                            items.push(Item {
                                code,
                                description,
                                price,
                            });
                        }
                    }
                }
            }
        }
        
        items
    }


}
