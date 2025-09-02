use std::collections::HashMap;
use std::io::{self};
use crate::MakeRule;

#[derive(Clone, Debug, Default)]
pub struct MakeGraph {
    pub default_target: String,
    pub dry: bool,
    rules: HashMap<String, MakeRule>,
}

impl MakeGraph {
    pub fn new() -> Self {
        MakeGraph {
            default_target: String::new(),
            dry: false,
            rules: HashMap::new(),
        }
    }

    pub fn set_dry(mut self, m: bool) -> Self {
        self.dry = m;
        self
    }

    pub fn get_rule_mut(&mut self, target: &str) -> Option<&mut MakeRule> {
        self.rules.get_mut(target)
    }

    pub fn add_rule(&mut self, result: String, rule: MakeRule) {
        self.rules.insert(result, rule);
    }

    pub fn run(&self, target: &str) -> Result<String, std::io::Error> {
        if let Some(rule) = self.rules.get(target) {
            rule.run(self)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No rule found"))
        }
    }
}
