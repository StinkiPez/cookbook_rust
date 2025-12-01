use cookbook::{CookbookManager, Recipe};
use std::path::PathBuf;

fn main() {
    let storage_path = PathBuf::from("data/recipes.json");
    let mut manager = CookbookManager::new(storage_path);
    
    // Example usage - you'll manage IDs yourself
    let recipe = Recipe::new("1".to_string(), "Pasta Carbonara".to_string());
    manager.add_recipe(recipe);
    
    println!("Cookbook initialized with {} recipes", manager.recipes.len());
}