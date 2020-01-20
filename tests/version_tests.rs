#[test]
fn version() {
    use tmux_interface::TmuxInterface;

    let mut tmux = TmuxInterface::new();
    let version = tmux.version().unwrap();
    assert_eq!(version.prog_name, "tmux");
    assert!(version.major >= 1);
}
