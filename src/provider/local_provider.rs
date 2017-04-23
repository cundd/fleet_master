use information::*;
use error::Error;

pub struct LocalProvider;

impl LocalProvider {
    pub fn get_information(self) -> Result<Information, Error> {
        Ok(Information::new_for_current_env())
    }
}

impl super::Provider for LocalProvider {
    fn new() -> Self {
        LocalProvider {}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_information_test() {
        let provider = LocalProvider {};

        let provider_information = provider.get_information().unwrap();
        let test_information = Information::new_for_current_env();

        assert_eq!(constants::PROTOCOL, test_information.fleet.protocol);
        assert_eq!(constants::PROVIDER_VERSION, test_information.fleet.provider_version);
        assert_eq!(constants::PROVIDER_NAME, test_information.fleet.provider_name);
        assert_eq!(test_information, provider_information);
    }
}
