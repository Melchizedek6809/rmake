use std::io;
use std::process::Command;

use crate::{MakeAtom, MakeGraph};

#[derive(Clone, Debug)]
pub enum MakeRecipeStep {
    Normal(MakeAtom),
    Silent(MakeAtom),
}

impl MakeRecipeStep {
    pub fn run(&self, graph: &MakeGraph) -> Result<Vec<String>, std::io::Error> {
        let mut ret = vec![];

        if let MakeRecipeStep::Normal(cmd) = self {
            let cmd = cmd.eval(graph);
            println!("{cmd}");
            ret.push(format!("{cmd}\n"));
        }

        match self {
            MakeRecipeStep::Normal(cmd) | MakeRecipeStep::Silent(cmd) => {
                let cmd = cmd.eval(graph);
                let output = Command::new("sh")
                    .args(["-c", &cmd])
                    .output()
                    .expect("Failed to execute command");

                if !output.status.success() {
                    match output.status.code() {
                        Some(_c) => Err(io::Error::other("Non zero exit code")),
                        None => Err(io::Error::other("Signal")),
                    }
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
