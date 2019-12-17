use super::application::*;
use super::platform::*;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct System {
    pub platform: Platform,
    pub application: Application,
}

impl System {
    pub fn new_for_current_env() -> Self {
        System {
            platform: Platform::new_for_current_env(),
            application: Application::new_for_current_env(),
        }
    }
}

//"system": {
//    "platform": {
//        "language": "php",
//        "version": "7.0.15",
//        "sapi": "cli",
//        "host": "dan.local",
//        "os": {
//            "vendor": "Darwin",
//            "version": "16.5.0",
//            "machine": "x86_64",
//            "info": "Darwin Kernel Version 16.5.0: Fri Mar  3 16:52:33 PST 2017; root:xnu-3789.51.2~3\/RELEASE_X86_64"
//        }
//    },
//    "application": {
//        "name": "TYPO3",
//        "version": "8.7.1-dev",
//        "meta": {
//            "branch": "8.7",
//            "applicationContext": "Production"
//        }
//    }
//},
