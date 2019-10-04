extern crate tmux_interface;

use self::tmux_interface::SendKeys;
use self::tmux_interface::TmuxInterface;
use super::error::Error;

//use super::project_cfg::ProjectCfg;

// TODO: #[skip_serializing_null] added in new serde
// XXX: cant use borrowed values [link](https://github.com/dtolnay/serde-yaml/issues/94)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeysCfg {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_lookup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouse_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_mode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_pane: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keys: Option<Vec<String>>,
}

impl Default for KeysCfg {
    fn default() -> Self {
        KeysCfg {
            repeat_count: None,
            target_pane: None,
            disable_lookup: None,
            mouse_event: None,
            reset: None,
            copy_mode: None,
            keys: None,
        }
    }
}

impl KeysCfg {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn send(&self, target_pane: &str) -> Result<(), Error> {
        let mut send_keys = Vec::new();
        if let Some(keys) = &self.keys {
            send_keys = keys.iter().map(|k| k.as_ref()).collect();
        }
        let send_keys = SendKeys {
            target_pane: Some(target_pane),
            key: send_keys,
            ..Default::default()
        };
        let tmux = TmuxInterface::new();
        //tmux.tmux = Some("tmux.sh");
        tmux.send_keys(&send_keys)?;
        Ok(())
    }
}
