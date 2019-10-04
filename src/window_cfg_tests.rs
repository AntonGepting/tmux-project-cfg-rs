#[test]
fn window_create() {
    use super::window_cfg::WindowCfg;
    use super::window_cfg::WindowOptionsCfg;

    let options = WindowOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let window_cfg = WindowCfg::new("asdf".to_string(), Some(options));
    let id = window_cfg.create("0", 5).unwrap();
    assert!(id > 0);
}

#[test]
fn window_parse() {
    use super::window_cfg::WindowCfg;

    let window_str = r#"
    window1:
        detached: true
        send_keys:
            keys: ["top"]
    "#;
    let window_cfg: WindowCfg = serde_yaml::from_str(window_str).unwrap();
    let id = window_cfg.create("0", 5).unwrap();
    assert!(id > 0);
}
