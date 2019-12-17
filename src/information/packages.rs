use std::collections::hash_map::IntoIter;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Packages {
    pub all: HashMap<String, Package>,
}

impl Packages {
    pub fn new() -> Self {
        Packages { all: HashMap::new() }
    }

    pub fn new_with_packages(packages: HashMap<String, Package>) -> Self {
        Packages { all: packages }
    }

    pub fn new_for_current_env() -> Self {
        Packages {
            all: HashMap::new()
        }
    }

    pub fn len(&self) -> usize {
        self.all.len()
    }

    pub fn iter(&self) -> Iter<String, Package> {
        self.all.iter()
    }

    pub fn into_iter(self) -> IntoIter<String, Package> {
        self.all.into_iter()
    }

    pub fn get(&self, key: &str) -> Option<&Package> {
        self.all.get(key)
    }
}

impl<'a> Index<&'a str> for Packages {
    type Output = Package;
    fn index(&self, s: &'a str) -> &Self::Output { // '
        &self.all[s]
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
