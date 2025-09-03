use std::path::PathBuf;

use rmake::MakeParser;

#[test]
fn hello_world() {
    let makefile = PathBuf::from("tests/makefiles/helloWorld.mk");
    let graph = MakeParser::from_file(&makefile).unwrap();
    println!("{graph:?}");

    assert!(graph.get_rule("hello").is_some());
    assert!(graph.get_rule("ferris").is_some());
    assert_eq!(graph.get_rule("ferris").unwrap().dependencies.len(), 1);
    assert!(graph.get_rule("ferris").unwrap().dependencies.contains("hello"));
    assert!(graph.get_rule("never").is_some());
    assert!(graph.get_rule(".PHONY").is_some());
    assert_eq!(graph.get_rule(".PHONY").unwrap().dependencies.len(), 3);
    assert!(graph.get_rule(".PHONY").unwrap().dependencies.contains("never"));
    assert!(graph.get_rule(".PHONY").unwrap().dependencies.contains("hello"));
    assert!(graph.get_rule(".PHONY").unwrap().dependencies.contains("ferris"));
    assert!(graph.get_rule("none").is_none());
    assert_eq!(
        graph.run(None).unwrap(),
        "echo \"Hello, World!\"\nHello, World!\n"
    );
    assert_eq!(
        graph.run(Some("ferris")).unwrap(),
        "echo \"Hello, World!\"\nHello, World!\nHello, Ferris!\n"
    );
}
