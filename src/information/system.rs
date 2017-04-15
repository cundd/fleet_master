use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Application {
    pub name: String,
    pub version: String,
    pub branch: String,
    pub meta: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Os {
    pub vendor: String,
    pub version: String,
    pub machine: String,
    pub info: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Platform {
    pub language: String,
    pub version: String,
    pub sapi: String,
    pub host: String,
    pub os: Os,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct System {
    pub platform: Platform,
    pub application: Application,
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
//        "branch": "8.7",
//        "meta": {
//            "applicationContext": "Production"
//        }
//    }
//},