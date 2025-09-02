use std::io;
use std::process::Command;

use crate::MakeGraph;

#[derive(Clone, Debug)]
pub enum MakeRecipeStep {
    Normal(String),
    Silent(String),
}

impl MakeRecipeStep {
    pub fn run(&self, _graph: &MakeGraph) -> Result<Vec<String>, std::io::Error> {
        let mut ret = vec![];

        if let MakeRecipeStep::Normal(cmd) = self {
            println!("{cmd}");
            ret.push(format!("{cmd}\n"));
        }

        match self {
            MakeRecipeStep::Normal(cmd) | MakeRecipeStep::Silent(cmd) => {
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
