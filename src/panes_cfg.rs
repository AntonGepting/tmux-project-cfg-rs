extern crate tmux_interface;

use self::tmux_interface::{Panes, TargetPaneEx, TargetWindowEx};
use super::error::Error;
use super::pane_cfg::{PaneCfg, PaneOptionsCfg};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
pub struct PanesCfg(Vec<PaneCfg>);

// %id
impl PanesCfg {
    // XXX: use of an array &[PaneCfg]
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, pane: PaneCfg) {
        self.0.push(pane)
    }

    // TODO: Some, None
    pub fn get(target_window: &TargetWindowEx, bitflags: usize) -> Option<PanesCfg> {
        let mut panes_cfg = PanesCfg::new();
        let mut _pane_cfg: PaneCfg;

        let panes = Panes::get(target_window, bitflags).ok();
        if let Some(panes) = panes {
            if !panes.0.is_empty() {
                for pane in panes {
                    let mut options = PaneOptionsCfg {
                        // detached by default
                        //detached: Some(true),
                        cwd: pane.current_path,
                        ..Default::default()
                    };

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

                    // XXX: is title set? otherwise then number
                    let pane_cfg = PaneCfg::new(pane.index.unwrap().to_string(), Some(options));
                    panes_cfg.push(pane_cfg);
                }
                return Some(panes_cfg);
            }
        }
        None
    }

    // TODO: defaults if needed
    pub fn create(&self, target_window: &TargetWindowEx) -> Result<Vec<usize>, Error> {
        let mut ids = Vec::new();
        for (i, pane_cfg) in self.0.iter().enumerate() {
            // if first pane different behavior (tmux creates one by creating a window)
            let (_key, first_value) = pane_cfg.0.iter().next().unwrap();
            if i == 0 {
                //let target_pane_str = format!("{}.0", target_window);
                //pane_cfg.rename(&target_window_str, &window.window_name.clone().unwrap());
                if let Some(ref send_keys) = first_value.as_ref().unwrap().send_keys {
                    send_keys.send(&TargetPaneEx::index(Some(&target_window), 0))?;
                }
            } else {
                let id = pane_cfg.create(&target_window)?;
                ids.push(id);
            }
            //if let Some(ref session_name) = map.keys().next() {
            //if let Some(ref start_directory) = self.start_directory {
            //session.cwd = Some(start_directory.to_string());
            //}
            //if let Some(ref session_name) = map.keys().next() {
            //session.session_name = Some(session_name.to_string());
            //}
            //if self.attach.is_none() && i == 0 {
            //self.attach = session.session_name;
            ////}
        }
        Ok(ids)
    }
}
