#[macro_use]
extern crate serde_derive;

pub mod error;
pub mod keys_cfg;
pub mod pane_cfg;
pub mod panes_cfg;
pub mod project_cfg;
pub mod session_cfg;
pub mod sessions_cfg;
pub mod window_cfg;
pub mod windows_cfg;

pub use self::error::Error;
pub use self::keys_cfg::KeysCfg;
pub use self::pane_cfg::PaneCfg;
pub use self::panes_cfg::PanesCfg;
pub use self::project_cfg::ProjectCfg;
pub use self::session_cfg::SessionCfg;
pub use self::sessions_cfg::SessionsCfg;
pub use self::window_cfg::WindowCfg;
pub use self::windows_cfg::WindowsCfg;

mod pane_cfg_tests;
mod panes_cfg_tests;
mod project_cfg_tests;
mod session_cfg_tests;
mod sessions_cfg_tests;
mod window_cfg_tests;
mod windows_cfg_tests;
//mod keys_cfg_tests;
mod error_tests;
