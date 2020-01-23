//#[test]
//fn project_create() {
//use tmux_project_cfg::project_cfg::ProjectCfg;

//let project_cfg = ProjectCfg {
//detached: Some(true),
//..Default::default()
//};
//let _id = project_cfg.create().unwrap();
//project_cfg.kill().unwrap();
////assert!(id > 0);
//}

//#[test]
//fn project_from_str() {
//use tmux_project_cfg::project_cfg::ProjectCfg;

//let project_str = r#"
//sessions:
//- mosaic:
//detached: true
//windows:
//- window1:
//- window2:
//- window3:
//"#;
//let project_cfg: ProjectCfg = serde_yaml::from_str(project_str).unwrap();
//let _id = project_cfg.create().unwrap();
//project_cfg.kill().unwrap();
////assert!(id > 0);
//}

//#[test]
//fn project_get() {
//use tmux_project_cfg::sessions_cfg::SessionsCfg;
//use tmux_project_cfg::{PANE_NONE, SESSION_NONE, WINDOW_NONE};

//let session_names = vec!["0"];

//let sessions_cfg =
//SessionsCfg::get(&session_names, SESSION_NONE, WINDOW_NONE, PANE_NONE).unwrap();
////let sessions_str = serde_yaml::to_string(&session_cfg).unwrap();
//}
