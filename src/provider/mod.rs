mod file_provider;
mod local_provider;
mod ssh_provider;

pub use self::local_provider::LocalProvider;
pub use self::ssh_provider::SshProvider;

pub trait Provider {
    fn new() -> Self;
}
