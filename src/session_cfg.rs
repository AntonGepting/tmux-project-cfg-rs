extern crate tmux_interface;
// TODO: in cfg session single then no list expected for deserialize, more then vector
//use std::time::Duration;

use self::tmux_interface::{NewSession, TmuxInterface, Sessions, SelectWindow};
//use super::keys::KeysCfg;

//use std::collections::HashMap;
//use std::process::Command;

use super::error::Error;
use super::keys_cfg::KeysCfg;
use super::windows_cfg::WindowsCfg;
use std::collections::BTreeMap;
//use super::tmux::*;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct SessionCfg(pub BTreeMap<String, Option<SessionOptionsCfg>>);

// TODO: #[skip_serializing_null] added in new serde
// XXX: cant use borrowed values [link](https://github.com/dtolnay/serde-yaml/issues/94)
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct SessionOptionsCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach_other: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_update_env: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attached: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_window: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<WindowsCfg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_keys: Option<KeysCfg>,
}

// $id
impl SessionCfg {
    pub fn new(name: String, options: Option<SessionOptionsCfg>) -> Self {
        let mut map = BTreeMap::new();
        map.insert(name, options);
        Self(map)
    }

    pub fn get_name(&self) -> Option<&str> {
        let (key, first_value) = self.0.iter().next().unwrap();
        let mut session_name = None;
        if let Some(value) = first_value {
            session_name = if value.session_name.is_some() {
                value.session_name.as_ref().map(|s| s.as_str())
            } else {
                Some(key)
            };
        };
        session_name
    }

    pub fn get_options(&self) -> Option<&SessionOptionsCfg> {
        let (_, first_value) = self.0.iter().next().unwrap();
        first_value.as_ref()
    }

    pub fn create(&self) -> Result<usize, Error> {
        let mut new_session = NewSession {
            detached: Some(true),
            print: Some(true),
            format: Some("#{session_id}"),
            ..Default::default()
        };
        let (_key, first_value) = self.0.iter().next().unwrap();
        let mut windows = None;
        let mut select_window = None;
        if let Some(value) = first_value {
            new_session.attach = value.attach;
            new_session.detached = value.detached;
            new_session.detach_other = value.detach_other;
            new_session.not_update_env = value.not_update_env;
            new_session.print = Some(true);
            new_session.cwd = value.cwd.as_ref().map(|s| s.as_str());
            new_session.format = Some("#{session_id}");
            new_session.window_name = value.window_name.as_ref().map(|s| s.as_str());
            new_session.session_name = self.get_name();
            new_session.group_name = value.group_name.as_ref().map(|s| s.as_str());
            new_session.width = value.width;
            new_session.height = value.height;
            new_session.shell_command = value.shell_command.as_ref().map(|s| s.as_str());
            windows = value.windows.as_ref();
            select_window = value.select_window.as_ref();
        };
        let tmux = TmuxInterface::new();
        let output = tmux.new_session(&new_session)?;
        let output_parts: Vec<&str> = output.split('\n').collect();
        let id = output_parts[0][1..].parse::<usize>()?;

        if let Some(windows) = windows {
            WindowsCfg::create(&windows, self.get_name().unwrap())?;
        }

        if let Some(select_window) = select_window {
            let select_window = SelectWindow {
                target_window: Some(&select_window),
                ..Default::default()
            };
            tmux.select_window(&select_window)?;
        }

        Ok(id)
    }

    pub fn get(session_name: &str) -> Result<SessionCfg, Error> {
        let tmux = TmuxInterface::new();
        if tmux.has_session(Some(session_name))? {
            let sessions = Sessions::get()?;
            for session in sessions {
                if session.name == Some(session_name.to_string()) {
                    let windows_cfg = WindowsCfg::get(session_name).ok();
                    let options = SessionOptionsCfg {
                        activity: session.activity.map(|t| t.as_millis()),
                        created: session.created.map(|t| t.as_millis()),
                        last_attached: session.last_attached.map(|t| t.as_millis()),
                        windows: windows_cfg,
                        ..Default::default()
                    };
                    let session_cfg = SessionCfg::new(session.name.unwrap(), Some(options));
                    return Ok(session_cfg);
                }
            }
        }
        Err(Error::new("iasdfsdf"))
    }

    // XXX: self check really needed?
    // XXX: opt
    pub fn exists(&self) -> Result<bool, Error> {
        let (_key, _first_value) = self.0.iter().next().unwrap();
        let session_name = self.get_name();
        let tmux = TmuxInterface::new();
        Ok(tmux.has_session(session_name)?)
    }

    pub fn attach(&self) -> bool {
        //let mut args: Vec<String> = Vec::new();
        //args.extend_from_slice(&[T_KEY.to_string(), name.to_string()]);
        //let mut tmux: Command = Tmux::new();
        //let output = tmux.exec(ATTACH_SESSION_CMD, &args);
        //output.success()
        false
    }

    //pub fn get_windows(&self) -> Result<SessionCfg, ()>{
    //let mut windows_cfg: Windows_= Vec::new();
    //let mut hashmap = HashMap::new();
    //let mut session_cfg: SessionCfg;
    //for session_name in sessions_names {
    //session_cfg = SessionCfg::get(session_name).unwrap();
    //hashmap.insert(session_cfg.clone().session_name.unwrap(), Some(session_cfg));
    //sessions_cfg.push(hashmap.clone());
    //project.sessions = Some(sessions_cfg.clone());
    //}
    //Ok(project)
    //}

    pub fn kill(&self) -> Result<(), Error> {
        let (_key, _first_value) = self.0.iter().next().unwrap();
        let session_name = self.get_name();
        let tmux = TmuxInterface::new();
        tmux.kill_session(None, None, session_name)?;
        Ok(())
    }

    //pub fn rename(&self, new_name: &str) {
    //if let Some(ref session_name) = self.session_name {
    //let tmux = TmuxInterface::new();
    //tmux.rename_session(Some(session_name), new_name).unwrap();
    //}

    // TODO: refactor Result
    //
    //pub fn list_windows(target_session: usize) -> Result<Vec<Window>, usize> {
    //let mut tmux = Command::new(TMUX);
    //let mut args: Vec<String> = Vec::new();
    //args.push(LIST_WINDOWS_CMD.to_string());
    //args.extend_from_slice(&[_F_KEY.to_string(), LIST_WINDOWS_FORMAT.to_string(),
    //T_KEY.to_string(), target_session.to_string()]);
    //let output = tmux.args(args).output().expect("failed to execute process");
    //let stdout = String::from_utf8_lossy(output.stdout.as_slice());
    //let mut windows: Vec<Window> = Vec::new();
    //for line in stdout.lines() {
    ////println!("line: {:?}", line);
    //windows.push(Window::get_from_string(line).unwrap());
    //}
    //Ok(windows)
    //}
}

impl SessionOptionsCfg {
    pub fn new() -> Self {
        Default::default()
    }
}
