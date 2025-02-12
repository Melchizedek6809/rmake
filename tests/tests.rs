use std::path::PathBuf;

use rmake::MakeGraph;

#[test]
fn hello_world() {
    let makefile = PathBuf::from("tests/makefiles/helloWorld.mk");
    assert_eq!(
        MakeGraph::new_run(&makefile).unwrap(),
        "echo \"Hello, World!\"\nHello, World!\n"
    );
}
