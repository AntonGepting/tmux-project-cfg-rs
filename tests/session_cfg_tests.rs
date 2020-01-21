#[test]
fn session_create() {
    use tmux_interface::TmuxInterface;
    use tmux_project_cfg::session_cfg::SessionCfg;
    use tmux_project_cfg::session_cfg::SessionOptionsCfg;

    const TEST_SESSION_NAME: &'static str = "session_create";

    let session_cfg = SessionCfg::new(
        TEST_SESSION_NAME.to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let id = session_cfg.create().unwrap();
    println!("session_create: {}", id);
    let mut tmux = TmuxInterface::new();
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();

    assert!(id >= 0);
}

#[test]
fn session_create_from_str() {
    use tmux_interface::TmuxInterface;
    //use crate::pane_cfg::PaneCfg;
    use tmux_project_cfg::session_cfg::SessionCfg;

    const TEST_SESSION_NAME: &'static str = "session_create_from_str";

    let session_str = r#"
    session_create_from_str:
        detached: true
    "#;
    let session_cfg: SessionCfg = serde_yaml::from_str(session_str).unwrap();
    let id = session_cfg.create().unwrap();
    println!("session_create_from_str: {}", id);
    let mut tmux = TmuxInterface::new();
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();

    assert!(id > 0);
}

#[test]
fn session_get() {
    use tmux_interface::TmuxInterface;
    use tmux_project_cfg::session_cfg::SessionCfg;
    use tmux_project_cfg::{PANE_ALL, SESSION_ALL, SESSION_NONE, WINDOW_ALL};

    const TEST_SESSION_NAME: &'static str = "session_get";

    let session_cfg = SessionCfg::get("0", SESSION_NONE, WINDOW_ALL, PANE_ALL).unwrap();
    let session_str = serde_yaml::to_string(&session_cfg).unwrap();
    let mut tmux = TmuxInterface::new();
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();
}
