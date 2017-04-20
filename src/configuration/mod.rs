mod ssh_configuration;
mod configuration_provider;

use std::collections::HashMap;

use self::ssh_configuration::SshConfiguration;

pub type Configuration = SshConfiguration;
pub type ConfigurationCollection = HashMap<String, Configuration>;

pub use self::configuration_provider::ConfigurationProvider;
