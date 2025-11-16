use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    Vietnamese,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub code: String,
    pub description: String,
    pub price: u32,
}
