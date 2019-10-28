#[test]
fn sessions_create() {
    use tmux_project_cfg::session_cfg::SessionCfg;
    use tmux_project_cfg::session_cfg::SessionOptionsCfg;
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
    let ids = sessions_cfg.create().unwrap();
    sessions_cfg.kill().unwrap();
    for id in ids {
        assert!(id > 0);
    }
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
    let ids = sessions_cfg.create().unwrap();
    sessions_cfg.kill().unwrap();
    for id in ids {
        assert!(id > 0);
    }
}

//#[test]
//fn sessions_get() {
//use tmux_project_cfg::sessions_cfg::SessionsCfg;
//use tmux_project_cfg::{PANE_NONE, SESSION_NONE, WINDOW_NONE};

//let sessions_names = vec!["session1", "session2", "session3"];
//let sessions_cfg =
//SessionsCfg::get(&sessions_names, SESSION_NONE, WINDOW_NONE, PANE_NONE).unwrap();
//let sessions_str = serde_yaml::to_string(&sessions_cfg).unwrap();
//print!("{}", sessions_str);
//}
