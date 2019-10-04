extern crate tmux_interface;

use self::tmux_interface::SplitWindow;
//use self::tmux_interface::SelectPane;
use self::tmux_interface::TmuxInterface;
use super::error::Error;
use super::keys_cfg::KeysCfg;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PaneCfg(pub BTreeMap<String, Option<PaneOptionsCfg>>);

// TODO: #[skip_serializing_null] added in new serde
// XXX: cant use borrowed values [link](https://github.com/dtolnay/serde-yaml/issues/94)
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct PaneOptionsCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_window: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<bool>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub print: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_pane: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<String>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_keys: Option<KeysCfg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl PaneCfg {
    pub fn new(name: String, options: Option<PaneOptionsCfg>) -> Self {
        let mut map = BTreeMap::new();
        map.insert(name, options);
        Self(map)
    }

    pub fn create(&self, target_window: &str) -> Result<usize, Error> {
        let tmux = TmuxInterface::new();
        // XXX: make it
        //let target_index = self.index.unwrap();
        // assumed: pane numbers always begin with 0
        //let target_pane_str = format!("{}.{}", target_window, target_index - 1);
        let target_pane_str = format!("{}.+1", target_window);
        let mut split_window = SplitWindow {
            detached: Some(true),
            print: Some(true),
            format: Some("#{pane_id}"),
            ..Default::default()
        };
        let (_key, first_value) = self.0.iter().next().unwrap();
        let mut send_keys = None;
        if let Some(value) = first_value {
            split_window.before = value.before;
            split_window.detached = value.detached;
            split_window.full = value.full_window;
            split_window.horizontal = value.horizontal;
            split_window.vertical = value.vertical;
            split_window.print = Some(true);
            split_window.cwd = value.cwd.as_ref().map(|s| s.as_str());
            split_window.size = value.size;
            split_window.percentage = value.percentage;
            split_window.target_pane = Some(&target_pane_str);
            split_window.shell_command = value.shell_command.as_ref().map(|s| s.as_str());
            split_window.format = Some("#{pane_id}");
            send_keys = value.send_keys.clone();
        }
        let output = tmux.split_window(&split_window)?;
        let output_parts: Vec<&str> = output.split('\n').collect();
        let id = output_parts[0][1..].parse::<usize>()?;
        let target_pane_str = format!("{}.%{}", target_window, id);
        send_keys.and_then(|k| k.send(&target_pane_str).ok());
        Ok(id)
    }

    //pub fn select(target_pane: &str) -> Result<(), Error> {
    //let tmux = TmuxInterface::new();
    //let select_pane = SelectPane {
    //target_pane: Some(target_pane),
    //..Default::default()
    //};
    //tmux.select_pane(&select_pane)?;
    //Ok(())
    //}
}

impl PaneOptionsCfg {
    pub fn new() -> Self {
        Default::default()
    }
}
