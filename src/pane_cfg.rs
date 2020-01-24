extern crate tmux_interface;

use self::tmux_interface::{PaneSize, Panes, SelectPane, SplitWindow, TmuxInterface};
use super::error::Error;
use super::keys_cfg::KeysCfg;
use std::collections::BTreeMap;
//use super::panes_cfg::PanesCfg;

// TODO: #[skip_serializing_null] added in new serde
// XXX: cant use borrowed values [link](https://github.com/dtolnay/serde-yaml/issues/94)
#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
pub struct PaneOptionsCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>, // set: SelectPane.?; get: Pane.active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<bool>, // set: SplitWindow.before; get:
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub index: Option<usize>, // set: - ; get: Pane.index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached: Option<bool>, // set: SplitWindow.detached; get: ? (derive Pane.active?)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_window: Option<bool>, // set: SplitWindow.full; get:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical: Option<bool>, // set: SplitWindow.vertical; get: ? (Pane.at_left, .at_bottom, .at_top, .at_right)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal: Option<bool>, // set: SplitWindow.horizontal; get: ? (Pane.at_left, .at_bottom, .at_top, .at_right)
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub print: Option<bool>, // set: SplitWindow.print; get:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>, // set: SplitWindow.cwd; get: Pane.current_path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<usize>, // set: SplitWindow.size; get: Pane.width or Pane.height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<usize>, // set: SplitWindow.percentage; get: ? (calculate?)
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub target_pane: Option<String>, // set: SplitWindow.target_pane; get:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<String>, // set: SplitWindow.shell_command; get: Pane.start_command
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub format: Option<String>, // set: SplitWindow.format; get: -
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_keys: Option<KeysCfg>, // set: SendKeys; get: ? (derive Pane.current_command?)
}

impl PaneOptionsCfg {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
pub struct PaneCfg(pub BTreeMap<String, Option<PaneOptionsCfg>>);

// %id
impl PaneCfg {
    pub fn new(name: String, options: Option<PaneOptionsCfg>) -> Self {
        let mut map = BTreeMap::new();
        map.insert(name, options);
        Self(map)
    }

    pub fn create(&self, target_window: &str) -> Result<usize, Error> {
        // target pane is next (+1) from current
        let target_pane_str = format!("{}.+1", target_window);
        // init new pane struct
        let mut split_window = SplitWindow {
            print: Some(true),
            format: Some("#{pane_id}"),
            ..Default::default()
        };
        // init tmux interface
        let mut tmux = TmuxInterface::new();
        //tmux.tmux = Some("./tmux_mock.sh");
        // init send keys struct
        let mut send_keys = None;
        // extract values from map
        let (_key, first_value) = self.0.iter().next().unwrap();
        if let Some(value) = first_value {
            split_window.before = value.before;
            // default all panes detached
            split_window.detached = Some(value.detached.unwrap_or(true));
            split_window.full = value.full_window;
            split_window.horizontal = value.horizontal;
            split_window.vertical = value.vertical;
            split_window.cwd = value.cwd.as_ref().map(|s| s.as_str());
            if let Some(size) = value.size {
                split_window.size = Some(PaneSize::Size(size));
            };
            if let Some(percentage) = value.percentage {
                split_window.size = Some(PaneSize::Percentage(percentage));
            };
            split_window.target_pane = Some(&target_pane_str);
            split_window.shell_command = value.shell_command.as_ref().map(|s| s.as_str());
            send_keys = value.send_keys.clone();
        }
        // create this pane
        let output = tmux.split_window(Some(&split_window))?;
        dbg!("output: {}", &output);
        // get new created pane id
        let output_parts: Vec<&str> = output.split('\n').collect();
        let id = output_parts[0][1..].parse::<usize>()?;
        let target_pane_str = format!("{}.%{}", target_window, id);
        // send keys to this pane by id
        send_keys.and_then(|k| k.send(&target_pane_str).ok());
        Ok(id)
    }

    pub fn select(target_pane: &str) -> Result<(), Error> {
        // init tmux interface
        let mut tmux = TmuxInterface::new();
        // init select pane struct
        let select_pane = SelectPane {
            target_pane: Some(target_pane),
            ..Default::default()
        };
        // select pane
        tmux.select_pane(Some(&select_pane))?;
        Ok(())
    }

    // XXX: Optimize/merge with panes_cfg::get_pane()?
    pub fn get(target_window: &str, pane_id: usize, bitflags: usize) -> Result<PaneCfg, Error> {
        // NOTE: not possible to get only a single pane?
        // get all panes
        let panes = Panes::get(target_window, bitflags)?;
        // save pane if found by id
        for pane in panes {
            if pane.id == Some(pane_id) {
                // init pane options struct
                let mut options = PaneOptionsCfg {
                    cwd: pane.current_path,
                    ..Default::default()
                };
                // save active status of the pane
                // do not save inactive status (inactive by default)
                if pane.active.unwrap_or(false) {
                    options.active = pane.active;
                }
                // split horizontal if pane doesn't touch left border (assume by default: !true)
                if !pane.at_left.unwrap_or(true) {
                    options.horizontal = Some(true);
                    options.size = pane.width;
                // split vertical if pane doesn't touch top border (assume by default: !true)
                } else if !pane.at_top.unwrap_or(true) {
                    options.vertical = Some(true);
                    options.size = pane.height;
                }
                // init and return this pane
                let pane_cfg = PaneCfg::new(pane.index.unwrap().to_string(), Some(options));
                return Ok(pane_cfg);
            }
        }
        // TODO: set right error
        Err(Error::new("pane not found?!"))
    }
}
