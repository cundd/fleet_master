use std::collections::HashMap;
use constants;

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub version: String,
    pub meta: HashMap<String, String>,
}

impl Application {
    pub fn new<S>(name: S, version: S, meta: HashMap<String, String>) -> Application where S: Into<String> {
        Application {
            name: name.into(),
            version: version.into(),
            meta: meta
        }
    }

    pub fn new_for_current_env() -> Application {
        Application::new(
            "fleet",
            constants::PROVIDER_VERSION,
            HashMap::new()
        )
    }
}
