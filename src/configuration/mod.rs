pub use self::configuration_collection::*;
pub use self::configuration_file_provider::detect_configuration_file;
pub use self::configuration_provider::ConfigurationProvider;
pub use self::helper::Helper;
use self::ssh_configuration::SshConfiguration;

mod configuration_collection;
mod configuration_file_provider;
mod configuration_provider;
mod helper;
mod ssh_configuration;

pub type Configuration = SshConfiguration;
