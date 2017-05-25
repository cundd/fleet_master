mod ssh_configuration;
mod configuration_collection;
mod configuration_provider;
mod configuration_file_provider;
mod helper;

use self::ssh_configuration::SshConfiguration;

pub type Configuration = SshConfiguration;

pub use self::configuration_provider::ConfigurationProvider;
pub use self::helper::Helper;
pub use self::configuration_collection::*;
pub use self::configuration_file_provider::detect_configuration_file;
