mod configuration_collection;
mod configuration_file_provider;
mod configuration_provider;
#[cfg(test)]
pub mod helper;
mod ssh_configuration;

pub use self::configuration_collection::*;
pub use self::configuration_file_provider::detect_configuration_file;
pub use self::configuration_provider::ConfigurationProvider;
use self::ssh_configuration::SshConfiguration;

pub type Configuration = SshConfiguration;
