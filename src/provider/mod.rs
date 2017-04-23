mod local_provider;
mod file_provider;
mod ssh_provider;

pub use self::local_provider::LocalProvider;
pub use self::file_provider::FileProvider;
pub use self::ssh_provider::SshProvider;

pub trait Provider {
    fn new() -> Self;
}
