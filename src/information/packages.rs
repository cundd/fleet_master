use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Package {
    pub key: String,
    pub version: String,
    pub description: String,
    pub state: String,
}

impl Package {
    pub fn is_active(&self) -> bool {
        return self.state == "active";
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Packages {
    pub all: HashMap<String, Package>,
}

impl Packages {
    pub fn new() -> Self {
        Packages { all: HashMap::new() }
    }

    pub fn new_for_current_env() -> Self {
        let mut packages = HashMap::new();
        packages.insert("test".to_owned(), Package {
            key: "test".to_owned(),
            version: "1.0".to_owned(),
            description: "desfa".to_owned(),
            state: "active".to_owned(),
        });


        Packages {
            all: packages
        }
    }

    pub fn len(&self) -> usize {
        self.all.len()
    }
}


//"all": {
//    "core": {
//        "key": "core",
//        "version": "8.7.0",
//        "description": "TYPO3 Core",
//        "state": "active"
//    },
//    "extbase": {
//        "key": "extbase",
//        "version": "8.7.0",
//        "description": "TYPO3 Core",
//        "state": "active"
//    },
//}