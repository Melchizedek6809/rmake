use std::io;

use crate::{MakeGraph, MakeRecipeStep};

#[derive(Clone, Debug, Default)]
pub struct MakeRecipe {
    pub steps: Vec<MakeRecipeStep>,
}

impl MakeRecipe {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn run(&self, graph: &MakeGraph) -> Result<String, io::Error> {
        let mut ret = vec![];
        for step in &self.steps {
            let mut r = step.run(graph)?;
            ret.append(&mut r);
        }
        Ok(ret.join(""))
    }

    pub fn push(&mut self, step: MakeRecipeStep) {
        self.steps.push(step);
    }
}
