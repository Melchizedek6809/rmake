use clap::Parser;
pub use rmake::MakeGraph;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    pub target: Option<String>,

    /// Sets a custom makefile
    #[arg(short, long, value_name = "FILE")]
    pub file: Option<PathBuf>,

    /// Allow N jobs at once; infinite jobs with no arg.
    #[arg(short, long, value_name = "N")]
    pub jobs: Option<u32>,
}

fn get_makefile(cli: &Cli) -> Option<PathBuf> {
    if let Some(file) = &cli.file {
        if file.exists() {
            return Some(file.clone());
        } else {
            eprintln!("{}: No such file or directory", file.to_str().unwrap());
            return None;
        }
    } else {
        let buf = PathBuf::from("GNUMakefile");
        if buf.exists() {
            return Some(buf);
        }
        let buf = PathBuf::from("Makefile");
        if buf.exists() {
            return Some(buf);
        }
        return None;
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(makefile) = get_makefile(&cli) {
        let graph = MakeGraph::new();
        let graph = graph.load(&makefile).unwrap();
        let target = cli.target.unwrap_or_else(|| graph.default_target.clone());
        graph.run(&target).unwrap();
    } else {
        eprintln!("rmake: *** No targets specified and no makefile found.  Stop.");
    }
}
