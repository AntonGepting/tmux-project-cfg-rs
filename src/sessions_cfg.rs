use super::error::Error;
use super::session_cfg::SessionCfg;
use super::tmux_interface::TargetSession;

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct SessionsCfg(Vec<SessionCfg>);

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
    pub fn create(&self) -> Result<Vec<usize>, Error> {
        let mut ids = Vec::new();
        for session_cfg in &self.0 {
            //if let Some(ref start_directory) = self.start_directory {
            //session.cwd = Some(start_directory.to_string());
            //}

            if !session_cfg.exists()? {
                let id = session_cfg.create()?;
                ids.push(id);
            } else {
                //error!("{}: ", LOG_SESSIONS_CFG_SESSION_ALREADY_EXISTS);
            }

            //if self.attach.is_none() && i == 0 {
            //self.attach = session.session_name;
            //}
        }
        Ok(ids)
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

    // XXX: struct bitflags
    // XXX: project bitflags
    pub fn get(
        target_sessions: &Vec<&TargetSession>,
        sbitflags: usize,
        wbitflags: usize,
        pbitflags: usize,
    ) -> Result<SessionsCfg, Error> {
        let mut sessions_cfg = SessionsCfg::new();
        let mut session_cfg: SessionCfg;
        for target_session in target_sessions {
            session_cfg = SessionCfg::get(target_session, sbitflags, wbitflags, pbitflags)?;
            sessions_cfg.push(session_cfg);
        }
        Ok(sessions_cfg)
    }
}
