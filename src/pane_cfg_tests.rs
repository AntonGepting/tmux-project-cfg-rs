#[test]
fn pane_create() {
    use self::tmux_interface::TmuxInterface;
    use super::pane_cfg::PaneCfg;
    use super::pane_cfg::PaneOptionsCfg;
    use self::tmux_interface::TmuxInterface;

    let options = PaneOptionsCfg {
        detached: Some(true),
        ..Default::default()
    };
    let target_window = "0:1";
    let pane_cfg = PaneCfg::new("asdf".to_string(), Some(options));
    let id = pane_cfg.create(target_window).unwrap();
    assert!(id > 0);
    let tmux = TmuxInterface::new();
    let target_pane = format!("{}.%{}", target_window, id);
    tmux.kill_pane(None, Some(&target_pane)).unwrap();
}

extern crate tmux_interface;

#[test]
fn pane_parse() {
    use self::tmux_interface::TmuxInterface;
    use super::pane_cfg::{PaneCfg, PaneOptionsCfg};
    use super::panes_cfg::PanesCfg;

    let pane_options = PaneOptionsCfg {
        detached: Some(true),
        horizontal: Some(true),
        percentage: Some(50),
        //send_keys:
        ..Default::default()
    };
    let pane = PaneCfg::new("pane".to_string(), Some(pane_options));
    let pane_str = serde_yaml::to_string(&pane).unwrap();
    //let pane_str = r#"
    //pane:
    //detached: true
    //horizontal: true
    //percentage: 50
    //send_keys:
    //keys: ["top"]
    //"#;
    let target_window = "0:1";
    let pane_cfg: PaneCfg = serde_yaml::from_str(&pane_str).unwrap();
    let id = pane_cfg.create(target_window).unwrap();
    assert!(id > 0);
    let target_pane = format!("{}.%{}", target_window, id);
    //let pane_result = PaneCfg::get(&target_window, id).unwrap();
    //assert_eq!(pane, pane_result);
    let tmux = TmuxInterface::new();
    tmux.kill_pane(None, Some(&target_pane)).unwrap();
}
