#[test]
fn session_create() {
    use super::session_cfg::SessionCfg;
    use super::session_cfg::SessionOptionsCfg;

    let options = SessionOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let session_cfg = SessionCfg::new("test".to_string(), Some(options));
    let id = session_cfg.create().unwrap();
    assert!(id > 0);
}

extern crate tmux_interface;

#[test]
fn session_parse() {
    //use super::pane_cfg::PaneCfg;
    use super::session_cfg::SessionCfg;

    let session_str = r#"
    1:
        detached: true
    "#;
    let session_cfg: SessionCfg = serde_yaml::from_str(session_str).unwrap();
    let id = session_cfg.create().unwrap();

    let _asdf = format!("0:1.%{}", id);
    assert!(id > 0);
}

#[test]
fn session_get() {
    use super::session_cfg::SessionCfg;
    let session_cfg = SessionCfg::get("0").unwrap();
    let session_str = serde_yaml::to_string(&session_cfg).unwrap();
    dbg!(session_str);
}
