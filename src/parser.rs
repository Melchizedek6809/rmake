use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{self, BufRead};

use crate::{MakeGraph, MakeRule};

pub struct MakeParser {

}

impl MakeParser {
    pub fn from_file(path: &PathBuf) -> Result<MakeGraph, std::io::Error> {
        Ok(Self::parse(MakeGraph::new(), path)?)
    }

    pub fn new_run(path: &PathBuf) -> Result<String, std::io::Error> {
        let g = Self::from_file(path)?;
        g.run(&g.default_target)
    }

    pub fn parse(mut graph: MakeGraph, path: &PathBuf) -> Result<MakeGraph, io::Error> {
        let lines = read_lines(path)?;

        let mut last_target = String::new();
        for line in lines.flatten() {
            if line.starts_with("\t") {
                if let Some(rule) = graph.get_rule_mut(&last_target) {
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
                        if graph.default_target.is_empty() && !target.starts_with(".") {
                            graph.default_target = target.to_string();
                        }
                        last_target = target.to_string();
                    }
                    if let Some(rule) = graph.get_rule_mut(&last_target) {
                        rule.add_dependency(parts[1].trim().to_owned());
                        continue;
                    } else {
                        let mut rule = MakeRule::new();
                        rule.add_dependency(parts[1].trim().to_owned());
                        graph.add_rule(target.to_string(), rule);
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

        Ok(graph)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
