extern crate tmux_interface;

use self::tmux_interface::{
    NewWindow, SelectWindow, TargetPaneEx, TargetSession, TargetWindowEx, TmuxInterface, Windows,
};

use super::error::Error;
use super::keys_cfg::KeysCfg;
use super::pane_cfg::PaneCfg;
use super::panes_cfg::PanesCfg;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct WindowCfg(pub BTreeMap<String, Option<WindowOptionsCfg>>);

impl WindowCfg {
    pub fn new(name: String, options: Option<WindowOptionsCfg>) -> Self {
        let mut map = BTreeMap::new();
        map.insert(name, options);
        Self(map)
    }

    pub fn create<'a>(&self, target_window: TargetWindowEx<'a>) -> Result<usize, Error> {
        let mut tmux = TmuxInterface::new();
        // init with default values
        let mut new_window = NewWindow {
            detached: Some(true),
            print: Some(true),
            format: Some("#{window_id}"),
            target_window: None,
            ..Default::default()
        };
        let (_key, first_value) = self.0.iter().next().unwrap();
        let mut send_keys = None;
        let mut panes = None;
        let mut select_pane = None;
        if let Some(value) = first_value {
            new_window.add = value.add;
            new_window.detached = value.detached;
            new_window.kill = value.kill;
            new_window.print = Some(true);
            new_window.cwd = value.cwd.as_ref().map(|s| s.as_str());
            new_window.print = Some(true);
            new_window.format = Some("#{window_id}");
            new_window.window_name = value.window_name.as_ref().map(|s| s.as_str());
            new_window.target_window = Some(&target_window);
            new_window.shell_command = value.shell_command.as_ref().map(|s| s.as_str());
            send_keys = value.send_keys.clone();
            panes = value.panes.as_ref();
            select_pane = value.select_pane;
        }
        let output = tmux.new_window(Some(&new_window))?;
        let output_parts: Vec<&str> = output.split('\n').collect();
        let id = output_parts[0][1..].parse::<usize>()?;
        let current_window = TargetWindowEx::id(target_window.session, id);
        send_keys.and_then(|k| k.send(&TargetPaneEx::index(Some(&current_window), 0)).ok());
        if let Some(panes_cfg) = panes {
            panes_cfg.create(&current_window)?;
        }
        if let Some(select_pane) = select_pane {
            PaneCfg::select(&TargetPaneEx::index(Some(&current_window), select_pane))?;
        }
        Ok(id)
    }

    pub fn exists() {
        unimplemented!();
    }

    pub fn select(target_window: &TargetWindowEx) -> Result<(), Error> {
        let mut tmux = TmuxInterface::new();
        let select_window = SelectWindow {
            target_window: Some(target_window),
            ..Default::default()
        };
        tmux.select_window(Some(&select_window))?;
        Ok(())
    }

    pub fn rename(&self, target_window: &TargetWindowEx, new_name: &str) -> Result<(), Error> {
        let mut tmux = TmuxInterface::new();
        tmux.rename_window(Some(target_window), new_name)?;
        Ok(())
    }

    // mb filter?
    pub fn get(
        target_session: &TargetSession,
        window_index: usize,
        wbitflags: usize,
        pbitflags: usize,
    ) -> Result<WindowCfg, Error> {
        let windows = Windows::get(target_session, wbitflags)?;
        for window in windows {
            if window.index == Some(window_index) {
                let panes_cfg = PanesCfg::get(
                    &TargetWindowEx::new(&window.name.clone().unwrap()),
                    pbitflags,
                );
                // TODO: none if not given bitflags
                let options = WindowOptionsCfg {
                    //detached: pane.detached,
                    active: window.active,
                    //index: window.index,
                    panes: panes_cfg,
                    ..Default::default()
                };
                let window_cfg = WindowCfg::new(window.name.unwrap(), Some(options));
                return Ok(window_cfg);
            }
        }
        Err(Error::new("window not found?!"))
    }
}

// TODO: #[skip_serializing_null] added in new serde
// XXX: cant use borrowed values [link](https://github.com/dtolnay/serde-yaml/issues/94)
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct WindowOptionsCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kill: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_index: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_current: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destroy_target: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_pane: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panes: Option<PanesCfg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_keys: Option<KeysCfg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl WindowOptionsCfg {
    pub fn new() -> Self {
        Default::default()
    }
}
