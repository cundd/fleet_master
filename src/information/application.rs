use crate::constants;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub name: String,
    pub version: String,
    pub install_mode: Option<String>,
    pub meta: HashMap<String, String>,
}

impl Application {
    pub fn new<S>(
        name: S,
        version: S,
        install_mode: Option<S>,
        meta: HashMap<String, String>,
    ) -> Self
    where
        S: Into<String>,
    {
        Application {
            name: name.into(),
            version: version.into(),
            install_mode: install_mode.map(|s| s.into()),
            meta,
        }
    }

    pub fn new_for_current_env() -> Self {
        Application::new(
            "fleet",
            constants::PROVIDER_VERSION,
            Some("binary"),
            HashMap::new(),
        )
    }
}
