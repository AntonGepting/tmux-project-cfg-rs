#[test]
fn version() {
    let mut tmux = TmuxInterface::new();
    let version = tmux.version().unwrap();
    assert_eq!(version.prog_name, "tmux");
    assert!(version.major >= 1);
}
