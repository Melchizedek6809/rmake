use std::io;
use std::process::Command;

use crate::MakeGraph;

#[derive(Clone, Debug)]
pub enum MakeRecipeSteps {
    Normal(String),
    Silent(String),
}

#[derive(Clone, Debug, Default)]
pub struct MakeRecipe {
    pub steps: Vec<MakeRecipeSteps>,
}

impl MakeRecipeSteps {
    pub fn run(&self, _graph: &MakeGraph) -> Result<Vec<String>, std::io::Error> {
        let mut ret = vec![];

        if let MakeRecipeSteps::Normal(cmd) = self {
            println!("{cmd}");
            ret.push(format!("{cmd}\n"));
        }

        match self {
            MakeRecipeSteps::Normal(cmd) | MakeRecipeSteps::Silent(cmd) => {
                let output = Command::new("sh")
                    .args(["-c", &cmd])
                    .output()
                    .expect("Failed to execute command");

                if !output.status.success() {
                    return match output.status.code() {
                        Some(_c) => Err(io::Error::new(io::ErrorKind::Other, "Non zero exit code")),
                        None => Err(io::Error::new(io::ErrorKind::Other, "Signal")),
                    };
                } else {
                    let stdout = std::str::from_utf8(output.stdout.as_slice())
                        .unwrap()
                        .to_owned();

                    let stderr = std::str::from_utf8(output.stderr.as_slice())
                        .unwrap()
                        .to_owned();

                    println!("{stdout}{stderr}");

                    ret.push(stdout);
                    ret.push(stderr);

                    Ok(ret)
                }
            }
        }
    }
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

    pub fn push(&mut self, line: String) {
        self.steps.push(MakeRecipeSteps::Normal(line));
    }
}
