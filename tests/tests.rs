use std::path::PathBuf;

use rmake::{MakeParser};

#[test]
fn hello_world() {
    let makefile = PathBuf::from("tests/makefiles/helloWorld.mk");
    assert_eq!(
        MakeParser::new_run(&makefile).unwrap(),
        "echo \"Hello, World!\"\nHello, World!\n"
    );
}
