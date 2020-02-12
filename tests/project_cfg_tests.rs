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
            - window2:
            - window3:
        "#;
    let project_cfg: ProjectCfg = serde_yaml::from_str(project_str).unwrap();
    assert!(project_cfg.create().is_ok());
    project_cfg.kill().unwrap();
}

//#[test]
//fn project_get() {
//use tmux_interface::TargetSession;
//use tmux_project_cfg::project_cfg::ProjectCfg;
//use tmux_project_cfg::sessions_cfg::SessionsCfg;
//use tmux_project_cfg::{PANE_ALL, SESSION_ALL, WINDOW_ALL};

//let project_str = r#"
//sessions:
//- project_get:
//detached: true
//windows:
//- window1:
//- window2:
//- window3:
//"#;

//let project_cfg: ProjectCfg = serde_yaml::from_str(project_str).unwrap();
//let target_session = TargetSession::new("project_get");
//let target_sessions = vec![&target_session];

////let project_cfg = ProjectCfg::get();
//assert!(SessionsCfg::get(&target_sessions, SESSION_ALL, WINDOW_ALL, PANE_ALL).is_ok());
//project_cfg.kill().unwrap();
//}
