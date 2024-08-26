pub use rmake::MakeGraph;

fn main() {
    MakeGraph::new_run("tests/makefiles/helloWorld.mk").unwrap();
}
