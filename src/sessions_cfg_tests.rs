#[test]
fn sessions_create() {
    use super::session_cfg::SessionCfg;
    use super::session_cfg::SessionOptionsCfg;
    use super::sessions_cfg::SessionsCfg;

    let options1 = SessionOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let options2 = SessionOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let options3 = SessionOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let session1_cfg = SessionCfg::new("test1".to_string(), Some(options1));
    let session2_cfg = SessionCfg::new("test2".to_string(), Some(options2));
    let session3_cfg = SessionCfg::new("test3".to_string(), Some(options3));

    let mut sessions_cfg: SessionsCfg = SessionsCfg::new();
    sessions_cfg.push(session1_cfg);
    sessions_cfg.push(session2_cfg);
    sessions_cfg.push(session3_cfg);
    let _id = sessions_cfg.create().unwrap();
    //assert!(id > 0);
}

//#[test]
//fn sessions_create() {
//use super::session_cfg::SessionCfg;
//use super::sessions_cfg::SessionsCfg;

//let session1_cfg = SessionCfg {
//detached: Some(true),
//..Default::default()
//};
//let session2_cfg = SessionCfg {
//detached: Some(true),
//..Default::default()
//};
//let mut sessions_cfg: SessionsCfg = MySessionsCfg::new();
////let mut panes_cfg: PanesCfg = Vec::new();
////let mut hashmap: HashMap<String, Option<SessionCfg>> = HashMap::new();
////hashmap.insert("asdf1".to_string(), Some(Session1_cfg.clone()));
////sessions_cfg.push(hashmap.clone());
//let mut hashmap: HashMap<String, Option<SessionCfg>> = HashMap::new();
//hashmap.insert("asdf1".to_string(), Some(session2_cfg.clone()));
//sessions_cfg.push(hashmap.clone());
//let mut hashmap: HashMap<String, Option<SessionCfg>> = HashMap::new();
//hashmap.insert("asdf2".to_string(), Some(session2_cfg.clone()));
//sessions_cfg.push(hashmap.clone());

//let _id = sessions_cfg.create().unwrap();
////assert!(id > 0);
////let id = pane.create("0:@3").unwrap();
////assert!(id > 0);
//}

//#[test]
//fn sessions_parse() {
////use super::pane_cfg::PaneCfg;

////let pane_str = r#"
////detached: true
////"#;
////let pane_cfg: PaneCfg = serde_yaml::from_str(pane_str).unwrap();
////let id = pane_cfg.create("").unwrap();
////assert!(id > 0);
//}

//////#[test]
//////fn create() {
//////use super::project_cfg::ProjectCfg;
////#[test]
////fn test() {
//////let mut sessions = sessionsCfg::new();
//////sessions.create();
////}

////#[test]
////fn create() {
////use super::project_cfg::ProjectCfg;
////use std::path::PathBuf;

////let project_cfg = ProjectCfg::read(&PathBuf::from("./tests/yml/test.min.mosaic.yml")).unwrap();
////let mut _sessions_cfg = project_cfg.sessions.unwrap();
////////sessions_cfg.create();
//////let mut sessions_cfg = sess
//////sessions_cfg.create();

//////let mut sessions = SessionsCfg::new();
//////sessions.create();
////}
