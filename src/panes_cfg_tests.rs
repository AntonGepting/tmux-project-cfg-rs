#[test]
fn panes_create() {
    use super::pane_cfg::PaneCfg;
    use super::pane_cfg::PaneOptionsCfg;
    use super::panes_cfg::PanesCfg;

    let options1_cfg = PaneOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let options2_cfg = PaneOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let options3_cfg = PaneOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let pane1_cfg = PaneCfg::new("1".to_string(), Some(options1_cfg));
    let pane2_cfg = PaneCfg::new("2".to_string(), Some(options2_cfg));
    let pane3_cfg = PaneCfg::new("3".to_string(), Some(options3_cfg));
    let mut panes_cfg: PanesCfg = PanesCfg::new();
    panes_cfg.push(pane1_cfg);
    panes_cfg.push(pane2_cfg);
    panes_cfg.push(pane3_cfg);
    let _id = panes_cfg.create("0:1").unwrap();
    //assert!(id > 0);
}

extern crate tmux_interface;

// FIXME: return symbol second pane
#[test]
fn panes_parse() {
    //use self::tmux_interface::Panes;
    use super::panes_cfg::PanesCfg;

    let pane_str = r#"
    - pane1:
        detached: true
        horizontal: true
        percentage: 50
        send_keys:
            keys: ["top"]

    - pane2:
        detached: true
        horizontal: true
        percentage: 50
        send_keys:
            keys: ["top"]

    - pane2:
        detached: true
        horizontal: true
        percentage: 50
        send_keys:
            keys: ["top"]
    "#;
    let panes_cfg: PanesCfg = serde_yaml::from_str(pane_str).unwrap();
    let _id = panes_cfg.create("0:1").unwrap();
    //let keke = format!("0:1.%{}", id);
    //dbg!(&keke);
    //let panes = Panes::get(&keke).unwrap();
    //dbg!(&panes[0]);
    //assert!(id > 0);
}
