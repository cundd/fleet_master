use super::*;
use constants;
//use super::fleet::Fleet;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Information {
    pub fleet: Fleet,
    pub system: System,
    pub packages: Packages,
}

impl Information {
    pub fn new_for_current_env() -> Self {
        Information {
            fleet: Fleet::new(constants::PROTOCOL, constants::PROVIDER_VERSION, constants::PROVIDER_NAME),
            packages: Packages::new(),
            system: System::new_for_current_env()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_for_current_env_test() {
        let info = Information::new_for_current_env();

        assert_eq!(constants::PROTOCOL, info.fleet.protocol);
        assert_eq!(constants::PROVIDER_VERSION, info.fleet.provider_version);
        assert_eq!(constants::PROVIDER_NAME, info.fleet.provider_name);

        assert_eq!(0, info.packages.all.len());
    }
}
