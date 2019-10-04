#[test]
fn pane_create() {
    use super::pane_cfg::PaneCfg;
    use super::pane_cfg::PaneOptionsCfg;

    let options = PaneOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let pane_cfg = PaneCfg::new("asdf".to_string(), Some(options));
    let id = pane_cfg.create("0:1").unwrap();
    assert!(id > 0);
}

extern crate tmux_interface;

#[test]
fn pane_parse() {
    use self::tmux_interface::Panes;
    use super::pane_cfg::PaneCfg;

    let pane_str = r#"
    pane:
        detached: true
        horizontal: true
        percentage: 50
        send_keys:
            keys: ["top"]
    "#;
    let pane_cfg: PaneCfg = serde_yaml::from_str(pane_str).unwrap();
    let id = pane_cfg.create("0:1").unwrap();
    let keke = format!("0:1.%{}", id);
    dbg!(&keke);
    let panes = Panes::get(&keke).unwrap();
    dbg!(&panes[0]);
    assert!(id > 0);
}
