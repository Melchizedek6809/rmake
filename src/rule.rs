use std::collections::HashSet;
use std::io;

use crate::{MakeGraph, MakeRecipe, MakeRecipeStep};

#[derive(Clone, Debug, Default)]
pub struct MakeRule {
    pub dependencies: HashSet<String>,
    pub recipe: MakeRecipe,
}

impl MakeRule {
    pub fn new() -> Self {
        MakeRule {
            dependencies: HashSet::new(),
            recipe: MakeRecipe::new(),
        }
    }

    pub fn run(&self, graph: &MakeGraph) -> Result<String, io::Error> {
        self.recipe.run(graph)
    }

    pub fn add_dependency(&mut self, dependency: String) {
        if dependency.is_empty() {
            return;
        }
        self.dependencies.insert(dependency);
    }

    pub fn add_recipe(&mut self, recipe: MakeRecipeStep) {
        self.recipe.push(recipe);
    }
}
