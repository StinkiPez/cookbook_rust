use std::fs;
use std::path::Path;
use crate::models::Recipe;

pub fn save_recipes(recipes: &[Recipe], path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(recipes)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn load_recipes(path: &Path) -> Result<Vec<Recipe>, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(path)?;
    let recipes = serde_json::from_str(&json)?;
    Ok(recipes)
}