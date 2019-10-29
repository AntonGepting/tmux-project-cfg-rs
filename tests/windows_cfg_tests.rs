#[test]
fn windows_create() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::window_cfg::{WindowCfg, WindowOptionsCfg};
    use tmux_project_cfg::windows_cfg::WindowsCfg;

    const TEST_SESSION_NAME: &'static str = "windows_create";

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let window1_cfg = WindowCfg::new(
        "1".to_string(),
        Some(WindowOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let window2_cfg = WindowCfg::new(
        "2".to_string(),
        Some(WindowOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let window3_cfg = WindowCfg::new(
        "3".to_string(),
        Some(WindowOptionsCfg {
            detached: Some(true),
            ..Default::default()
        }),
    );
    let mut windows_cfg: WindowsCfg = WindowsCfg::new();
    windows_cfg.push(window1_cfg);
    windows_cfg.push(window2_cfg);
    windows_cfg.push(window3_cfg);
    let ids = windows_cfg.create(TEST_SESSION_NAME).unwrap();

    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();

    for id in ids {
        assert!(id > 0);
    }
}

#[test]
fn windows_from_str() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::windows_cfg::WindowsCfg;

    const TEST_SESSION_NAME: &'static str = "windows_create_from_str";

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let windows_str = r#"
      - window1:
          detached: true
      - window2:
          detached: true
      - window3:
          detached: true
    "#;
    let windows_cfg: WindowsCfg = serde_yaml::from_str(windows_str).unwrap();
    let ids = windows_cfg.create(TEST_SESSION_NAME).unwrap();

    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();

    for id in ids {
        assert!(id > 0);
    }
}

#[test]
fn windows_get() {
    use tmux_interface::{NewSession, TmuxInterface};
    use tmux_project_cfg::windows_cfg::WindowsCfg;
    use tmux_project_cfg::{PANE_ALL, WINDOW_ALL};

    const TEST_SESSION_NAME: &'static str = "windows_get";

    let tmux = TmuxInterface::new();
    let new_session = NewSession {
        detached: Some(true),
        session_name: Some(TEST_SESSION_NAME),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();

    let windows_cfg = WindowsCfg::get("0", WINDOW_ALL, PANE_ALL).unwrap();
    let windows_str = serde_yaml::to_string(&windows_cfg).unwrap();
    //print!("{}", windows_str);
    tmux.kill_session(None, None, Some(TEST_SESSION_NAME))
        .unwrap();
}
