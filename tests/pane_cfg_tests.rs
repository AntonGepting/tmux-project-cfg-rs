#[test]
fn pane_create() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::pane_cfg::{PaneCfg, PaneOptionsCfg};

    const TEST_SESSION_NAME: &'static str = "pane_create";

    let options = PaneOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let pane_cfg = PaneCfg::new("test_pane".to_string(), Some(options));

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    assert!(pane_cfg.create(&format!("{}:^", TEST_SESSION_NAME)).is_ok());
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();
}

#[test]
fn pane_create_from_str() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::pane_cfg::PaneCfg;

    const TEST_SESSION_NAME: &'static str = "pane_create_from_str";

    let pane_str = r#"
        pane:
            detached: true
            horizontal: true
            percentage: 50
            send_keys:
            keys: ["top"]
        "#;
    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
    let pane_cfg: PaneCfg = serde_yaml::from_str(&pane_str).unwrap();
    assert!(pane_cfg.create(&format!("{}:1", TEST_SESSION_NAME)).is_ok());
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();
}

//#[test]
//fn pane_get() {
//use tmux_interface::{NewSession, TmuxInterface};
//use tmux_project_cfg::pane_cfg::PaneCfg;
//use tmux_project_cfg::PANE_ALL;

//const TEST_SESSION_NAME: &'static str = "pane_get";

//let tmux = TmuxInterface::new();
//let new_session = NewSession {
//detached: Some(true),
//session_name: Some(TEST_SESSION_NAME),
//..Default::default()
//};

//tmux.new_session(Some(&new_session)).unwrap();
//let pane_cfg = PaneCfg::get(&format!("{}:1", TEST_SESSION_NAME), 0, PANE_ALL).unwrap();
//let pane_str = serde_yaml::to_string(&pane_cfg).unwrap();
//dbg!(pane_str);
//tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
//.unwrap();
//}
