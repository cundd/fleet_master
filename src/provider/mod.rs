pub use self::file_provider::FileProvider;
pub use self::local_provider::LocalProvider;
pub use self::ssh_provider::SshProvider;

mod file_provider;
mod local_provider;
mod ssh_provider;

pub trait Provider {
    fn new() -> Self;
}
