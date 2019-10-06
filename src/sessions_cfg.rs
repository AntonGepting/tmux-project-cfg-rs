use super::error::Error;
use super::session_cfg::SessionCfg;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct SessionsCfg(Vec<SessionCfg>);

//impl IntoIterator for SessionsCfg {
//type Item = SessionCfg;
//type IntoIter = ::std::vec::IntoIter<Self::Item>;

//fn into_iter(self) -> Self::IntoIter {
//self.0.into_iter()
//}
//}

// $id
impl SessionsCfg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, session: SessionCfg) {
        self.0.push(session)
    }

    // TODO: defaults if needed
    // TODO: Result
    // XXX: clone() - really needed?
    pub fn create(&self) -> Result<(), Error> {
        for session_cfg in &self.0 {
            //if let Some(session) = session_cfg.values().next().unwrap_or(&None) {
            //if let Some(ref start_directory) = self.start_directory {
            //session.cwd = Some(start_directory.to_string());
            //}

            if !session_cfg.exists()? {
                session_cfg.create()?;
            } else {
                //error!("{}: ", LOG_SESSIONS_CFG_SESSION_ALREADY_EXISTS);
            }

            //if self.attach.is_none() && i == 0 {
            //self.attach = session.session_name;
            //}
        }
        Ok(())
    }

    // TODO: Result
    pub fn kill(&self) -> Result<(), Error> {
        for session_cfg in &self.0 {
            if session_cfg.exists()? {
                session_cfg.kill()?;
            }
        }
        Ok(())
    }

    pub fn get(sessions_names: Vec<&str>) -> Result<SessionsCfg, Error> {
        let mut sessions_cfg = SessionsCfg::new();
        let mut session_cfg: SessionCfg;
        for session_name in sessions_names {
            session_cfg = SessionCfg::get(session_name)?;
            sessions_cfg.push(session_cfg);
        }
        Ok(sessions_cfg)
    }
}
