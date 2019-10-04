#[test]
fn project_create() {
    use super::project_cfg::ProjectCfg;

    let project_cfg = ProjectCfg {
        detached: Some(true),
        ..Default::default()
    };
    let _id = project_cfg.create().unwrap();
    //assert!(id > 0);
}

#[test]
fn project_parse() {
    use super::project_cfg::ProjectCfg;

    let project_str = r#"
    sessions:
      - mosaic:
          detached: true
          windows:
            - window1:
            - window2:
            - window3:
    "#;
    let project_cfg: ProjectCfg = serde_yaml::from_str(project_str).unwrap();
    let _id = project_cfg.create().unwrap();
    //assert!(id > 0);
}
