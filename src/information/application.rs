use std::collections::HashMap;

use crate::constants;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Application {
    pub name: String,
    pub version: String,
    pub meta: HashMap<String, String>,
}

impl Application {
    pub fn new<S>(name: S, version: S, meta: HashMap<String, String>) -> Self where S: Into<String> {
        Application {
            name: name.into(),
            version: version.into(),
            meta: meta,
        }
    }

    pub fn new_for_current_env() -> Self {
        Application::new(
            "fleet",
            constants::PROVIDER_VERSION,
            HashMap::new(),
        )
    }
}
