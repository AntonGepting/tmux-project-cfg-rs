#[test]
fn window_create() {
    use tmux_interface::{
        NewSession, TargetSession, TargetWindowEx, TargetWindowToken, TmuxInterface,
    };
    use tmux_project_cfg::window_cfg::{WindowCfg, WindowOptionsCfg};

    const TEST_SESSION_NAME: &'static str = "window_create";
    const TEST_WINDOW_NAME: &'static str = "window1";
    let target_session = TargetSession::exact_name(TEST_SESSION_NAME);
    let target_window = TargetWindowEx::token(Some(&target_session), TargetWindowToken::Next(None));

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let options = WindowOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let window_cfg = WindowCfg::new(TEST_WINDOW_NAME.to_string(), Some(options));
    assert!(window_cfg.create(target_window).is_ok());
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}

#[test]
fn window_create_from_str() {
    use tmux_interface::{
        NewSession, TargetSession, TargetWindowEx, TargetWindowToken, TmuxInterface,
    };
    use tmux_project_cfg::window_cfg::WindowCfg;

    const TEST_SESSION_NAME: &'static str = "window_create_from_str";
    //const TEST_WINDOW_NAME: &'static str = "window1";
    let target_session = TargetSession::new(TEST_SESSION_NAME);
    let target_window = TargetWindowEx::token(Some(&target_session), TargetWindowToken::Next(None));

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let window_str = r#"
        window1:
            detached: true
            #send_keys:
                #keys: ["top"]
    "#;
    let window_cfg: WindowCfg = serde_yaml::from_str(window_str).unwrap();
    assert!(window_cfg.create(target_window).is_ok());
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}

#[test]
fn window_get() {
    use tmux_interface::{NewSession, TargetSession, TmuxInterface};
    use tmux_project_cfg::window_cfg::WindowCfg;
    use tmux_project_cfg::{PANE_ALL, WINDOW_ALL};

    const TEST_SESSION_NAME: &'static str = "window_get";
    let target_session = TargetSession::new(TEST_SESSION_NAME);

    let mut tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    assert!(WindowCfg::get(&target_session, 1, WINDOW_ALL, PANE_ALL).is_ok());
    tmux.kill_session(None, None, Some(&target_session))
        .unwrap();
}
