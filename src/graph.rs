use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

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

    pub fn from_file(path: &PathBuf) -> Result<Self, std::io::Error> {
        MakeGraph::new().load(path)
    }

    pub fn add_rule(&mut self, result: String, rule: MakeRule) {
        self.rules.insert(result, rule);
    }

    pub fn new_run(path: &PathBuf) -> Result<String, std::io::Error> {
        let g = MakeGraph::from_file(path)?;
        g.run(&g.default_target)
    }

    pub fn load(mut self, path: &PathBuf) -> Result<Self, std::io::Error> {
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
                let parts: Vec<&str> = line.split(":").collect();
                if parts.len() == 2 {
                    let target = parts[0].trim();
                    if !target.is_empty() {
                        if self.default_target.is_empty() && !target.starts_with(".") {
                            self.default_target = target.to_string();
                        }
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
                } else {
                    let parts: Vec<&str> = line.split("=").collect();
                    if parts.len() >= 2 {
                        continue;
                    } else {
                        if line.trim().len() == 0 {
                            continue;
                        }
                        println!("Invalid line: {}", line);
                        continue;
                    }
                }
            }
        }
        Ok(self)
    }

    pub fn run(&self, target: &str) -> Result<String, std::io::Error> {
        if let Some(rule) = self.rules.get(target) {
            rule.run(self)
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "No rule found"))
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
