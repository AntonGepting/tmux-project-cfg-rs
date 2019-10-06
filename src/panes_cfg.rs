extern crate tmux_interface;

use self::tmux_interface::Panes;
use super::error::Error;
use super::pane_cfg::{PaneCfg, PaneOptionsCfg};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
pub struct PanesCfg(Vec<PaneCfg>);

//impl IntoIterator for PanesCfg {
//type Item = HashMap<String, Option<PaneCfg>>;
//type IntoIter = ::std::vec::IntoIter<Self::Item>;

//fn into_iter(self) -> Self::IntoIter {
//self.0.into_iter()
//}
//}
//

// %id
impl PanesCfg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, pane: PaneCfg) {
        self.0.push(pane)
    }

    pub fn get(target_window: &str) -> Result<PanesCfg, Error> {
        let mut panes_cfg = PanesCfg::new();
        let mut _pane_cfg: PaneCfg;

        let panes = Panes::get(target_window)?;
        for pane in panes {
            let options = PaneOptionsCfg {
                index: pane.index,
                cwd: pane.current_path,
                shell_command: pane.current_command,
                active: pane.active,
                ..Default::default()
            };
            // XXX: is title set? otherwise then number
            let pane_cfg = PaneCfg::new(pane.title.unwrap(), Some(options));
            panes_cfg.push(pane_cfg);
        }
        Ok(panes_cfg)
    }

    // TODO: defaults if needed
    pub fn create(&self, target_window: &str) -> Result<(), Error> {
        for (i, pane_cfg) in self.0.iter().enumerate() {
            // if first pane different behavior (tmux creates one by creating a window)
            let (_key, first_value) = pane_cfg.0.iter().next().unwrap();
            if i == 0 {
                let target_pane_str = format!("{}.0", target_window);
                //pane_cfg.rename(&target_window_str, &window.window_name.clone().unwrap());
                if let Some(ref send_keys) = first_value.as_ref().unwrap().send_keys {
                    send_keys.send(&target_pane_str)?;
                }
            } else {
                pane_cfg.create(target_window)?;
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
        Ok(())
    }
}
