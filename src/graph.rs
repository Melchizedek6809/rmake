use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::MakeRule;

#[derive(Clone, Debug, Default)]
pub struct MakeGraph {
    default_target: String,
    rules: HashMap<String, MakeRule>,
}

impl MakeGraph {
    pub fn new() -> Self {
        MakeGraph {
            default_target: String::new(),
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, result: String, rule: MakeRule) {
        self.rules.insert(result, rule);
    }

    pub fn new_run(path: &str) -> Result<(), std::io::Error> {
        let g = MakeGraph::new();
        let g = g.load(path)?;
        g.run(&g.default_target)
    }

    pub fn new_mock_run(path: &str) -> Result<String, std::io::Error> {
        let g = MakeGraph::new();
        let g = g.load(path)?;
        g.mock_run(&g.default_target)
    }

    pub fn load(mut self, path: &str) -> Result<Self, std::io::Error> {
        let lines = read_lines(path)?;
        let mut last_target = String::new();
        for line in lines.flatten() {
            if line.starts_with("\t") {
                if let Some(rule) = self.rules.get_mut(&last_target) {
                    rule.add_recipe(line.trim().to_owned());
                    continue;
                } else {
                    println!("Invalid line: {}", line);
                    continue;
                }
            } else {
                let parts:Vec<&str> = line.split(":").collect();
                if parts.len() != 2 {
                    println!("Invalid line: {}", line);
                    continue;
                }
                let target = parts[0].trim();
                if !target.is_empty() {
                    self.default_target = target.to_string();
                    last_target = target.to_string();
                }
                if let Some(rule) = self.rules.get_mut(&last_target) {
                    rule.add_dependency(parts[1].trim().to_owned());
                    continue;
                } else {
                    let mut rule = MakeRule::new();
                    rule.add_dependency(parts[1].trim().to_owned());
                    self.add_rule(target.to_string(), rule);
                }
            }
        }
        Ok(self)
    }

    pub fn run(&self, target: &str) -> Result<(), std::io::Error> {
        if let Some(rule) = self.rules.get(target) {
            rule.run()
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No rule found"))
        }
    }

    pub fn mock_run(&self, target: &str) -> Result<String, std::io::Error> {
        if let Some(rule) = self.rules.get(target) {
            let res = rule.run_mock()?;
            Ok(res.join(""))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No rule found"))
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}