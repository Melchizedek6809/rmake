use std::path::PathBuf;

use rmake::MakeParser;

#[test]
fn hello_world() {
    let makefile = PathBuf::from("tests/makefiles/helloWorld.mk");

    let graph = MakeParser::from_file(&makefile).unwrap();

    assert!(graph.get_rule("hello").is_some());
    assert!(graph.get_rule("ferris").is_some());
    assert!(graph.get_rule("none").is_none());
    assert_eq!(
        graph.run(None).unwrap(),
        "echo \"Hello, World!\"\nHello, World!\n"
    );
    assert_eq!(graph.run(Some("ferris")).unwrap(), "Hello, Ferris!\n");
}
