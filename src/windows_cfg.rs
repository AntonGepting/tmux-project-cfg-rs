extern crate tmux_interface;

use self::tmux_interface::{TmuxInterface, TmuxOption, Windows};
use super::error::Error;
use super::panes_cfg::PanesCfg;
use super::window_cfg::{WindowCfg, WindowOptionsCfg};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct WindowsCfg(Vec<WindowCfg>);

// @id
impl WindowsCfg {
    pub fn new() -> Self {
        Default::default()
    }

    // TODO: defaults if needed
    // XXX: ref window_cfg
    pub fn create(&self, target_session: &str) -> Result<Vec<usize>, Error> {
        let mut ids = Vec::new();
        for (i, window_cfg) in self.0.iter().enumerate() {
            //if window.window_name.is_none() {
            //window.window_name = Some(key.to_string());
            //}
            //}
            let (key, first_value) = window_cfg.0.iter().next().unwrap();
            // if custom index is given
            if let Some(idx) = first_value.as_ref().unwrap().index {
                window_cfg.clone().create(target_session, idx)?;
            // use default index
            } else {
                // if first window different behavior (because tmux creates window by creating a session)
                // XXX: and index is not given
                if i == 0 {
                    let target_window_str = format!("{}:^", target_session);
                    // rename first
                    window_cfg.clone().rename(&target_window_str, &key)?;
                //.rename(&target_window_str, &first_value.as_ref().unwrap().window_name.clone().unwrap());
                // create manually panes
                //window_cfg
                //.panes
                //.as_ref()
                //.and_then(|panes| panes.create(&target_window_str).ok());
                // second and others windows
                } else {
                    let base_index = TmuxOption::get_int("base-index")?;
                    let id = window_cfg.clone().create(target_session, i + base_index)?;
                    ids.push(id);
                }
                //if let Some(ref start_directory) = self.start_directory {
                //session.cwd = Some(start_directory.to_string());
                //}
                //}
            }
        }
        Ok(ids)
    }

    pub fn get(
        target_session: &str,
        wbitflags: usize,
        pbitflags: usize,
    ) -> Result<WindowsCfg, Error> {
        let mut tmux = TmuxInterface::new();
        let mut windows_cfg = WindowsCfg::new();
        let mut window_cfg: WindowCfg;
        if tmux.has_session(Some(target_session))? {
            let windows = Windows::get(&target_session, wbitflags).unwrap();
            for window in windows {
                let target_window =
                    format!("{}:{}", &target_session, &window.clone().index.unwrap());
                let panes_cfg = PanesCfg::get(&target_window, pbitflags);
                let mut options = WindowOptionsCfg {
                    //activity: window.activity.map(|t| t.as_millis()),
                    // index can be derived by default from Vec()
                    //index: window.index,
                    panes: panes_cfg,
                    ..Default::default()
                };

                // do not save inactive status (inactive by default)
                if window.active.unwrap_or(false) {
                    options.active = window.active;
                }

                window_cfg = WindowCfg::new(window.name.unwrap(), Some(options));
                windows_cfg.push(window_cfg);
            }
            return Ok(windows_cfg);
        }
        Err(Error::new("asdf"))
    }

    pub fn push(&mut self, window: WindowCfg) {
        self.0.push(window)
    }
}
