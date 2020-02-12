#[test]
fn sessions_create() {
    use tmux_project_cfg::session_cfg::{SessionCfg, SessionOptionsCfg};
    use tmux_project_cfg::sessions_cfg::SessionsCfg;

    let session1_cfg = SessionCfg::new(
        "test1".to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let session2_cfg = SessionCfg::new(
        "test2".to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let session3_cfg = SessionCfg::new(
        "test3".to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let mut sessions_cfg: SessionsCfg = SessionsCfg::new();
    sessions_cfg.push(session1_cfg);
    sessions_cfg.push(session2_cfg);
    sessions_cfg.push(session3_cfg);
    assert!(sessions_cfg.create().is_ok());
    sessions_cfg.kill().unwrap();
}

#[test]
fn sessions_from_str() {
    use tmux_project_cfg::sessions_cfg::SessionsCfg;

    let sessions_str = r#"
      - session1:
          detached: true
      - session2:
          detached: true
      - session3:
          detached: true
    "#;
    let sessions_cfg: SessionsCfg = serde_yaml::from_str(sessions_str).unwrap();
    assert!(sessions_cfg.create().is_ok());
    sessions_cfg.kill().unwrap();
}

#[test]
fn sessions_get() {
    use tmux_interface::TargetSession;
    use tmux_project_cfg::session_cfg::{SessionCfg, SessionOptionsCfg};
    use tmux_project_cfg::sessions_cfg::SessionsCfg;
    use tmux_project_cfg::{PANE_ALL, SESSION_ALL, WINDOW_ALL};

    const TEST_SESSION_NAME1: &'static str = "sessions_get1";
    const TEST_SESSION_NAME2: &'static str = "sessions_get2";
    const TEST_SESSION_NAME3: &'static str = "sessions_get3";

    let session1_cfg = SessionCfg::new(
        TEST_SESSION_NAME1.to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let session2_cfg = SessionCfg::new(
        TEST_SESSION_NAME2.to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let session3_cfg = SessionCfg::new(
        TEST_SESSION_NAME3.to_string(),
        Some(SessionOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let mut sessions_cfg: SessionsCfg = SessionsCfg::new();
    sessions_cfg.push(session1_cfg);
    sessions_cfg.push(session2_cfg);
    sessions_cfg.push(session3_cfg);
    assert!(sessions_cfg.create().is_ok());

    let target_session1 = TargetSession::new(TEST_SESSION_NAME1);
    let target_session2 = TargetSession::new(TEST_SESSION_NAME2);
    let target_session3 = TargetSession::new(TEST_SESSION_NAME3);
    let sessions_names = vec![&target_session1, &target_session2, &target_session3];
    assert!(SessionsCfg::get(&sessions_names, SESSION_ALL, WINDOW_ALL, PANE_ALL).is_ok());
    sessions_cfg.kill().unwrap();
}
