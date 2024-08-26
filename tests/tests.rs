use rmake::MakeGraph;

#[test]
fn hello_world() {
    assert_eq!(MakeGraph::new_mock_run("tests/makefiles/helloWorld.mk").unwrap(), "echo \"Hello, World!\"\nHello, World!\n");
}