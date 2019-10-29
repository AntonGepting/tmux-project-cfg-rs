#[test]
fn panes_create() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::pane_cfg::{PaneCfg, PaneOptionsCfg};
    use tmux_project_cfg::panes_cfg::PanesCfg;

    const TEST_SESSION_NAME: &'static str = "panes_create";

    let pane1_cfg = PaneCfg::new(
        "1".to_string(),
        Some(PaneOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let pane2_cfg = PaneCfg::new(
        "2".to_string(),
        Some(PaneOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let pane3_cfg = PaneCfg::new(
        "3".to_string(),
        Some(PaneOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let mut panes_cfg: PanesCfg = PanesCfg::new();
    panes_cfg.push(pane1_cfg);
    panes_cfg.push(pane2_cfg);
    panes_cfg.push(pane3_cfg);

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let ids = panes_cfg
        .create(&format!("{}:1", TEST_SESSION_NAME))
        .unwrap();

    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();

    for id in ids {
        assert!(id > 0);
    }
}

#[test]
fn panes_create_from_str() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::panes_cfg::PanesCfg;

    const TEST_SESSION_NAME: &'static str = "panes_create_from_str";

    let pane_str = r#"
    - pane1:
        detached: true
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

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };

    tmux.new_session(Some(&new_session)).unwrap();
    let ids = panes_cfg
        .create(&format!("{}:1", TEST_SESSION_NAME))
        .unwrap();

    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();

    for id in ids {
        assert!(id > 0);
    }
}

#[test]
fn panes_get() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::pane_cfg::{PaneCfg, PaneOptionsCfg};
    use tmux_project_cfg::panes_cfg::PanesCfg;
    use tmux_project_cfg::PANE_ALL;

    const TEST_SESSION_NAME: &'static str = "panes_get";

    let pane1_cfg = PaneCfg::new(
        "0".to_string(),
        Some(PaneOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let pane2_cfg = PaneCfg::new(
        "1".to_string(),
        Some(PaneOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let pane3_cfg = PaneCfg::new(
        "2".to_string(),
        Some(PaneOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let mut panes_cfg_orig: PanesCfg = PanesCfg::new();
    panes_cfg_orig.push(pane1_cfg);
    panes_cfg_orig.push(pane2_cfg);
    panes_cfg_orig.push(pane3_cfg);

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };

    tmux.new_session(Some(&new_session)).unwrap();
    panes_cfg_orig
        .create(&format!("{}:1", TEST_SESSION_NAME))
        .unwrap();
    let panes_cfg = PanesCfg::get(&format!("{}:1", TEST_SESSION_NAME), PANE_ALL).unwrap();
    //let panes_str = serde_yaml::to_string(&panes_cfg).unwrap();
    //dbg!(panes_str);
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();
    //assert_eq!(panes_cfg, panes_cfg_orig);
}
