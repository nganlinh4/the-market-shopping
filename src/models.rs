use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Vietnamese,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark,
    Warm,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Warm => "Warm",
        }
    }

    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::Light => ThemeColors {
                background: egui::Color32::from_rgb(248, 249, 250),
                card: egui::Color32::from_rgb(255, 255, 255),
                primary: egui::Color32::from_rgb(74, 144, 226),
                success: egui::Color32::from_rgb(80, 227, 194),
                danger: egui::Color32::from_rgb(233, 78, 119),
                text_primary: egui::Color32::from_rgb(51, 51, 51),
                text_secondary: egui::Color32::from_rgb(119, 119, 119),
            },
            Theme::Dark => ThemeColors {
                background: egui::Color32::from_rgb(30, 30, 35),
                card: egui::Color32::from_rgb(45, 45, 52),
                primary: egui::Color32::from_rgb(100, 160, 255),
                success: egui::Color32::from_rgb(100, 240, 200),
                danger: egui::Color32::from_rgb(255, 100, 120),
                text_primary: egui::Color32::from_rgb(240, 240, 245),
                text_secondary: egui::Color32::from_rgb(180, 180, 190),
            },
            Theme::Warm => ThemeColors {
                background: egui::Color32::from_rgb(255, 250, 240),
                card: egui::Color32::from_rgb(255, 253, 248),
                primary: egui::Color32::from_rgb(200, 120, 60),
                success: egui::Color32::from_rgb(140, 180, 80),
                danger: egui::Color32::from_rgb(220, 80, 80),
                text_primary: egui::Color32::from_rgb(60, 50, 40),
                text_secondary: egui::Color32::from_rgb(140, 130, 115),
            },
        }
    }

    pub fn all() -> [Theme; 3] {
        [Theme::Light, Theme::Dark, Theme::Warm]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub background: egui::Color32,
    pub card: egui::Color32,
    pub primary: egui::Color32,
    pub success: egui::Color32,
    pub danger: egui::Color32,
    pub text_primary: egui::Color32,
    pub text_secondary: egui::Color32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub code: String,
    pub description: String,
    pub price: u32,
    #[serde(default)]
    pub category: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemCategory {
    Pork,
    Chicken,
    Duck,
    Beef,
    Seafood,
    Sauces,
    Vegetables,
    Other,
}

impl ItemCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            ItemCategory::Pork => "🐖 Pork",
            ItemCategory::Chicken => "🐓 Chicken",
            ItemCategory::Duck => "🦆 Duck",
            ItemCategory::Beef => "🐂 Beef",
            ItemCategory::Seafood => "🐟 Seafood",
            ItemCategory::Sauces => "🥫 Sauces",
            ItemCategory::Vegetables => "🍐 Vegetables",
            ItemCategory::Other => "Other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pork" | "🐖 pork" => ItemCategory::Pork,
            "chicken" | "🐓 chicken" => ItemCategory::Chicken,
            "duck" | "🦆 duck" => ItemCategory::Duck,
            "beef" | "🐂 beef" => ItemCategory::Beef,
            "seafood" | "🐟 seafood" => ItemCategory::Seafood,
            "sauces" | "🥫 sauces" => ItemCategory::Sauces,
            "vegetables" | "🍐 vegetables" => ItemCategory::Vegetables,
            _ => ItemCategory::Other,
        }
    }

    pub fn all() -> [ItemCategory; 8] {
        [
            ItemCategory::Pork,
            ItemCategory::Chicken,
            ItemCategory::Duck,
            ItemCategory::Beef,
            ItemCategory::Seafood,
            ItemCategory::Sauces,
            ItemCategory::Vegetables,
            ItemCategory::Other,
        ]
    }
}
