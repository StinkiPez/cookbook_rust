use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub servings: u32,
}

impl Recipe {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            ingredients: Vec::new(),
            instructions: Vec::new(),
            servings: 1,
        }
    }
}