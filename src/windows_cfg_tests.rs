#[test]
fn windows_create() {
    use super::window_cfg::WindowCfg;
    use super::window_cfg::WindowOptionsCfg;
    use super::windows_cfg::WindowsCfg;

    let options1_cfg = WindowOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let options2_cfg = WindowOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let options3_cfg = WindowOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let window1_cfg = WindowCfg::new("1".to_string(), Some(options1_cfg));
    let window2_cfg = WindowCfg::new("2".to_string(), Some(options2_cfg));
    let window3_cfg = WindowCfg::new("3".to_string(), Some(options3_cfg));
    let mut windows_cfg: WindowsCfg = WindowsCfg::new();
    windows_cfg.push(window1_cfg);
    windows_cfg.push(window2_cfg);
    windows_cfg.push(window3_cfg);
    let _id = windows_cfg.create("1").unwrap();
    //assert!(id > 0);
    //let id = pane.create("0:@3").unwrap();
    //assert!(id > 0);
}

#[test]
fn windows_parse() {
    use super::windows_cfg::WindowsCfg;

    let windows_str = r#"
      - window1:
          detached: true
      - window2:
          detached: true
      - window3:
          detached: true
    "#;
    let windows_cfg: WindowsCfg = serde_yaml::from_str(windows_str).unwrap();
    let _id = windows_cfg.create("1").unwrap();
    //assert!(id > 0);
}
