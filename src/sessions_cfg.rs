use super::error::Error;
use super::session_cfg::{SessionCfg, SessionOptionsCfg};
use super::tmux_interface::{Sessions, TargetSession, SESSION_NAME};
use super::windows_cfg::WindowsCfg;

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
    // TODO: multiple requesting only possible (cant get a single pane, window, session?!)
    pub fn get(
        target_sessions: &Vec<&TargetSession>,
        sbitflags: usize,
        wbitflags: usize,
        pbitflags: usize,
    ) -> Result<SessionsCfg, Error> {
        let mut sessions_cfg = SessionsCfg::new();
        //let mut session_cfg: SessionCfg;
        let sessions = Sessions::get(sbitflags | SESSION_NAME)?;

        for target_session in target_sessions {
            //session_cfg = SessionCfg::get(target_session, sbitflags, wbitflags, pbitflags)?;
            // TODO: ref not clone?!
            for session in sessions.clone() {
                if session.name == Some(target_session.to_string()) {
                    let windows_cfg = WindowsCfg::get(target_session, wbitflags, pbitflags).ok();
                    let options = SessionOptionsCfg {
                        activity: session.activity.map(|t| t.as_millis()),
                        created: session.created.map(|t| t.as_millis()),
                        last_attached: session.last_attached.map(|t| t.as_millis()),
                        windows: windows_cfg,
                        ..Default::default()
                    };
                    let session_cfg = SessionCfg::new(session.name.unwrap(), Some(options));
                    sessions_cfg.push(session_cfg);
                }
            }
        }
        Ok(sessions_cfg)
    }
}
