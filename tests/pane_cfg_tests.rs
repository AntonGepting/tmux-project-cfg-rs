#[test]
fn pane_create() {
    use tmux_interface::{
        NewSession, TargetSession, TargetWindowEx, TargetWindowToken, TmuxInterface,
    };
    use tmux_project_cfg::pane_cfg::{PaneCfg, PaneOptionsCfg};

    const TEST_SESSION_NAME: &'static str = "pane_create";
    let target_session = TargetSession::new(TEST_SESSION_NAME);
    let target_window = TargetWindowEx::token(Some(&target_session), TargetWindowToken::Start);

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
    assert!(pane_cfg.create(&target_window).is_ok());
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}

#[test]
fn pane_create_from_str() {
    use tmux_interface::{
        NewSession, TargetSession, TargetWindowEx, TargetWindowToken, TmuxInterface,
    };
    use tmux_project_cfg::pane_cfg::PaneCfg;

    const TEST_SESSION_NAME: &'static str = "pane_create_from_str";
    let target_session = TargetSession::new(TEST_SESSION_NAME);
    let target_window = TargetWindowEx::token(Some(&target_session), TargetWindowToken::Start);

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
    assert!(pane_cfg.create(&target_window).is_ok());
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}

#[test]
fn pane_get() {
    use tmux_interface::{
        NewSession, TargetSession, TargetWindowEx, TargetWindowToken, TmuxInterface,
    };
    use tmux_project_cfg::pane_cfg::PaneCfg;
    use tmux_project_cfg::PANE_ALL;

    const TEST_SESSION_NAME: &'static str = "pane_get";

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };

    let target_window = TargetWindowEx::token(
        Some(&TargetSession::Raw(TEST_SESSION_NAME)),
        TargetWindowToken::Start,
    );

    tmux.new_session(Some(&new_session)).unwrap();
    assert!(PaneCfg::get(&target_window, 0, PANE_ALL).is_ok());
    tmux.kill_session(None, None, Some(&TargetSession::Raw(TEST_SESSION_NAME)))
        .unwrap();
}
