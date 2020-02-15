#[test]
fn project_create() {
    use tmux_project_cfg::project_cfg::ProjectCfg;

    let project_cfg = ProjectCfg {
        detached: Some(true),
        ..Default::default()
    };
    assert!(project_cfg.create().is_ok());
    project_cfg.kill().unwrap();
}

#[test]
fn project_from_str() {
    use tmux_project_cfg::project_cfg::ProjectCfg;

    let project_str = r#"
        sessions:
            - project_from_str:
                detached: true
                windows:
                    - window1:
                        detached: true
                    - window2:
                        detached: true
                    - window3:
                        detached: true
    "#;
    let project_cfg: ProjectCfg = serde_yaml::from_str(project_str).unwrap();
    assert!(project_cfg.create().is_ok());
    project_cfg.kill().unwrap();
}

#[test]
fn project_get() {
    use tmux_interface::TargetSession;
    use tmux_project_cfg::project_cfg::ProjectCfg;
    use tmux_project_cfg::{PANE_ALL, SESSION_ALL, WINDOW_ALL};

    let project_str = r#"
        sessions:
            - project_get1:
                detached: true
            - project_get2:
                detached: true
    "#;

    let project_cfg: ProjectCfg = serde_yaml::from_str(project_str).unwrap();
    assert!(project_cfg.create().is_ok());

    let target_session1 = TargetSession::Raw("project_get1");
    let target_session2 = TargetSession::Raw("project_get2");
    let target_sessions = vec![&target_session1, &target_session2];

    assert!(ProjectCfg::get(&target_sessions, SESSION_ALL, WINDOW_ALL, PANE_ALL).is_ok());

    project_cfg.kill().unwrap();
}
