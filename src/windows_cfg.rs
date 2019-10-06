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
    pub fn create(&self, target_session: &str) -> Result<(), Error> {
        for (i, window_cfg) in self.0.iter().enumerate() {
            //if let Some(key) = window_cfg.keys().next() {
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
                    window_cfg.clone().create(target_session, i + base_index)?;
                }
                //if let Some(ref start_directory) = self.start_directory {
                //session.cwd = Some(start_directory.to_string());
                //}
                //}
            }
        }
        Ok(())
    }

    pub fn get(target_session: &str) -> Result<WindowsCfg, Error> {
        let tmux = TmuxInterface::new();
        let mut windows_cfg = WindowsCfg::new();
        let mut window_cfg: WindowCfg;
        if tmux.has_session(Some(target_session))? {
            let windows = Windows::get(&target_session).unwrap();
            for window in windows {
                let panes_cfg = PanesCfg::get(&window.clone().name.unwrap()).ok();
                let options = WindowOptionsCfg {
                    activity: window.activity.map(|t| t.as_millis()),
                    index: window.index,
                    active: window.active,
                    panes: panes_cfg,
                    ..Default::default()
                };
                window_cfg = WindowCfg::new(window.name.unwrap(), Some(options));
                windows_cfg.push(window_cfg);
            }
            return Ok(windows_cfg);
        }
        Err(Error::new("asdf"))
    }

    //pub fn get(target_session: &str) -> Result<WindowsCfg, ()> {
    //let tmux = Tmux::new(None);
    //let mut windows_cfg: WindowsCfg = Vec::new();
    //let mut window_cfg: WindowCfg;
    //if tmux.has_session(target_session).unwrap() {
    //let windows_str = tmux.list_windows(false, Some(LIST_WINDOWS_FORMAT), Some(target_session)).unwrap();
    //let windows = Windows::from_str(&windows_str).unwrap();
    //for window in windows {
    //let _panes_cfg = PaneCfg::get(&window.index.to_string()).ok();
    //let mut hashmap: HashMap<String, Option<WindowCfg>> = HashMap::new();
    //window_cfg = WindowCfg {
    //activity: Some(window.activity.as_millis()),
    //index: Some(window.index),
    //window_name: Some(window.clone().name),
    //active: if window.active { Some(true) } else { None },
    //..Default::default()
    //};
    //hashmap.insert(window.name.clone(), Some(window_cfg));
    //windows_cfg.push(hashmap.clone());
    //}
    //return Ok(windows_cfg);
    //}
    //return Err(());
    //}

    pub fn push(&mut self, window: WindowCfg) {
        self.0.push(window)
    }
}
