//#[macro_use]
//extern crate serde_derive;
extern crate serde_yaml;
extern crate tmux_interface;

use super::error::Error;
use super::sessions_cfg::SessionsCfg;
use std::fs;
use std::path::PathBuf;

use self::tmux_interface::AttachSession;
use self::tmux_interface::TmuxInterface;

// TODO: #[skip_serializing_null] added in new serde
// XXX: cant use borrowed values [link](https://github.com/dtolnay/serde-yaml/issues/94)
#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct ProjectCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detached: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<SessionsCfg>,
}

impl ProjectCfg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create(&self) -> Result<(), Error> {
        if let Some(ref s) = self.sessions {
            s.create()?;
        }
        Ok(())
    }

    pub fn kill(&self) -> Result<(), Error> {
        if let Some(ref s) = self.sessions {
            s.kill()?;
        }
        Ok(())
    }

    pub fn read(filename: &PathBuf) -> Result<ProjectCfg, Error> {
        let content = fs::read_to_string(filename)?;
        let config = serde_yaml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_str(project_str: &str) -> Result<ProjectCfg, Error> {
        let config = serde_yaml::from_str(project_str)?;
        Ok(config)
    }

    pub fn to_string(&self) -> Result<String, Error> {
        let project_str = serde_yaml::to_string(self)?;
        Ok(project_str)
    }

    pub fn attach(&self) -> Result<(), Error> {
        if !self.detached.unwrap_or(false) && self.attach.is_some() {
            let tmux = TmuxInterface::new();
            let attach_session = AttachSession {
                target_session: self.attach.as_ref().map(|s| s.as_ref()),
                ..Default::default()
            };
            tmux.attach_session(&attach_session)?;
        }
        Ok(())
    }

    pub fn get(sessions_names: Vec<&str>) -> Result<ProjectCfg, Error> {
        let mut project = ProjectCfg::new();
        let sessions_cfg = SessionsCfg::get(sessions_names)?;
        project.sessions = Some(sessions_cfg);
        Ok(project)
    }

    pub fn write(&self, filename: &PathBuf) -> Result<(), Error> {
        //let config_str = toml::to_string(self).unwrap();
        let project_str = serde_yaml::to_string(self)?;
        fs::write(filename, project_str.as_bytes())?;
        Ok(())
    }
}
