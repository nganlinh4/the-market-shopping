use crate::models::{Item, Language, ItemCategory, Theme};
use crate::raw_data;
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
        // Try to load CSV file from various locations (optional)
        let csv_content = Self::load_csv_file();
        
        // Use embedded raw data (no external file dependencies)
        let raw_content = raw_data::RAW_LIST_DATA.to_string();
        eprintln!("Loaded embedded raw list data successfully");

        let items = if csv_content.is_empty() {
            // If no CSV file found, generate items from embedded raw data
            let _raw_items = raw_data::extract_initial_items(); // Can be used for future features
            let generated_items = Self::parse_raw_list(&raw_content);
            eprintln!("Generated {} items from embedded raw data", generated_items.len());
            generated_items
        } else {
            Self::parse_items(&csv_content)
        };
        
        eprintln!("Loaded {} items into catalog", items.len());

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

    fn load_csv_file() -> String {
        // Get the directory where the executable is located
        let exe_dir = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();

        // Try to load CSV file from various locations
        let possible_paths = [
            exe_dir.join("items.csv"),                       // Same directory as executable
            exe_dir.parent().unwrap().join("items.csv"),     // Parent directory
            exe_dir.parent().unwrap().parent().unwrap().join("items.csv"), // Grandparent (project root)
            std::path::PathBuf::from("items.csv"),           // Current working directory
        ];
        
        for path in &possible_paths {
            if path.exists() {
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        eprintln!("Loaded items.csv from: {:?}", path);
                        return content;
                    },
                    Err(e) => eprintln!("Failed to read items.csv from {:?}: {}", path, e),
                }
            }
        }
        
        String::new() // Return empty string if file not found
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
                let price_str = if let Some(_arrow_pos) = after_slash.find('→') {
                    // Find the character boundary after the arrow
                    let arrow_char_indices: Vec<_> = after_slash.char_indices().collect();
                    if let Some(&(arrow_byte_pos, _)) = arrow_char_indices.iter().find(|&&(_, c)| c == '→') {
                        let after_arrow = &after_slash[arrow_byte_pos + '→'.len_utf8()..];
                        after_arrow.trim()
                    } else {
                        after_slash
                    }
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
        // Get the directory where the executable is located
        let exe_dir = std::env::current_exe()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .to_path_buf();

        let save_path = exe_dir.join("items.csv");
        
        let mut wtr = csv::Writer::from_writer(vec![]);
        for item in &self.items {
            wtr.serialize(item)?;
        }
        let data = String::from_utf8(wtr.into_inner()?)?;
        std::fs::write(&save_path, data)?;
        eprintln!("Successfully saved items to: {:?}", save_path);
        Ok(())
    }


}
