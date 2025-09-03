use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::{MakeAtom, MakeGraph, MakeRecipeStep, MakeRule};

pub struct MakeParser {
    last_target: String,
}

impl MakeParser {
    fn new() -> Self {
        MakeParser {
            last_target: "".to_string(),
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<MakeGraph, std::io::Error> {
        Self::parse(MakeGraph::new(), path)
    }

    pub fn new_run(path: &PathBuf, target: Option<&str>) -> Result<String, std::io::Error> {
        Self::from_file(path)?.run(target)
    }

    fn parse_recipe(&mut self, graph: &mut MakeGraph, line: &str) -> Result<(), io::Error> {
        if let Some(rule) = graph.get_rule_mut(&self.last_target) {
            let line = line.trim();
            if let Some(line) = line.strip_prefix("@") {
                let step = MakeRecipeStep::Silent(String::from(line));
                rule.add_recipe(step);
            } else {
                let step = MakeRecipeStep::Normal(String::from(line));
                rule.add_recipe(step);
            }
        } else {
            println!("Invalid line: {}", line);
        }
        Ok(())
    }

    fn parse_rule(&mut self, graph: &mut MakeGraph, line: &str) -> Result<(), io::Error> {
        let parts: Vec<&str> = line.split(":").collect();
        if parts.len() == 2 {
            let targets = parts[0].trim();
            let targets = MakeAtom::new(targets);
            let targets = targets.eval(graph);

            let dependencies = parts[1].trim();
            let dependencies = MakeAtom::new(dependencies);
            let dependencies = dependencies.eval(graph);

            for target in targets.split_ascii_whitespace() {
                if !target.is_empty() {
                    if graph.default_target.is_empty() && !target.starts_with(".") {
                        graph.default_target = target.to_string();
                    }
                    self.last_target = target.to_string();
                }

                if let None = graph.get_rule(target) {
                    graph.add_rule(target.to_string(), MakeRule::new());
                }

                for dependency in dependencies.split_ascii_whitespace() {
                    let dependency = dependency.to_owned();

                    if let Some(rule) = graph.get_rule_mut(target) {
                        rule.add_dependency(dependency);
                    } else {
                        panic!("Can't add rule!");
                    }
                }
            }

            Ok(())
        } else {
            let parts: Vec<&str> = line.split("=").collect();
            if parts.len() >= 2 {
                Ok(())
            } else {
                if !line.trim().is_empty() {
                    println!("Invalid line: {}", line);
                }
                Ok(())
            }
        }
    }

    fn parse_line(&mut self, graph: &mut MakeGraph, line: &str) -> Result<(), io::Error> {
        if line.starts_with("\t") {
            self.parse_recipe(graph, line)
        } else if line.starts_with("#") {
            Ok(()) // Comment
        } else {
            self.parse_rule(graph, line)
        }
    }

    pub fn parse(mut graph: MakeGraph, path: &PathBuf) -> Result<MakeGraph, io::Error> {
        let mut parser = MakeParser::new();
        let lines = read_lines(path)?;

        for line in lines.map_while(Result::ok) {
            parser.parse_line(&mut graph, &line)?;
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
