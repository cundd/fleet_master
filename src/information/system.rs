use super::application::*;
use super::platform::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct System {
    pub platform: Platform,
    pub application: Application,
}

impl System {
    pub fn new_for_current_env() -> Self {
        System {
            platform: Platform::new_for_current_env(),
            application: Application::new_for_current_env(),
        }
    }
}
