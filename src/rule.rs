use std::collections::HashSet;
use std::io;
use std::process::{Command, Stdio};

#[derive(Clone, Debug, Default)]
pub struct MakeRule {
    pub dependencies: HashSet<String>,
    pub recipe: Vec<String>,
}

impl MakeRule {
    pub fn new() -> Self {
        MakeRule {
            dependencies: HashSet::new(),
            recipe: Vec::new(),
        }
    }

    pub fn run(&self) -> Result<(), io::Error> {
        for line in &self.recipe {
            println!("{}", line);
            let output = Command::new("sh")
                .args(["-c", line])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .expect("Failed to execute command");
            if !output.status.success() {
                return match output.status.code() {
                    Some(_c) => Err(io::Error::new(io::ErrorKind::Other, "Non zero exit code")),
                    None => Err(io::Error::new(io::ErrorKind::Other, "Signal")),
                };
            }
        }
        return Ok(());
    }

    pub fn run_mock(&self) -> Result<Vec<String>, io::Error> {
        let mut ret: Vec<String> = Vec::new();
        for line in &self.recipe {
            ret.push(line.clone());
            ret.push("\n".to_string());
            let output = Command::new("sh")
                .args(["-c", line])
                .output()
                .expect("Failed to execute command");
            if !output.status.success() {
                return match output.status.code() {
                    Some(_c) => Err(io::Error::new(io::ErrorKind::Other, "Non zero exit code")),
                    None => Err(io::Error::new(io::ErrorKind::Other, "Signal")),
                };
            }
            ret.push(
                std::str::from_utf8(output.stdout.as_slice())
                    .unwrap()
                    .to_owned(),
            );
            ret.push(
                std::str::from_utf8(output.stderr.as_slice())
                    .unwrap()
                    .to_owned(),
            );
        }
        Ok(ret)
    }

    pub fn add_dependency(&mut self, dependency: String) {
        if dependency.is_empty() {
            return;
        }
        self.dependencies.insert(dependency);
    }

    pub fn add_recipe(&mut self, recipe: String) {
        self.recipe.push(recipe);
    }
}
