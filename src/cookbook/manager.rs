use crate::models::Recipe;
use std::path::PathBuf;

pub struct CookbookManager {
    pub recipes: Vec<Recipe>,
    pub storage_path: PathBuf,
}

impl CookbookManager {
    pub fn new(storage_path: PathBuf) -> Self {
        Self {
            recipes: Vec::new(),
            storage_path,
        }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.push(recipe);
    }

    pub fn get_recipe(&self, id: &str) -> Option<&Recipe> {
        self.recipes.iter().find(|r| r.id == id)
    }
}