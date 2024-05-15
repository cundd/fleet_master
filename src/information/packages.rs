use serde::{Deserialize, Serialize};
use std::collections::hash_map::IntoIter;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Package {
    pub key: String,
    pub version: String,
    pub description: String,
    pub state: String,
}

#[cfg(test)]
impl Package {
    pub fn is_active(&self) -> bool {
        self.state == "active"
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Packages {
    pub all: HashMap<String, Package>,
}

impl Packages {
    pub fn new() -> Self {
        Packages {
            all: HashMap::new(),
        }
    }

    pub fn new_with_packages(packages: HashMap<String, Package>) -> Self {
        Packages { all: packages }
    }

    pub fn new_for_current_env() -> Self {
        Packages {
            all: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.all.len()
    }

    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }

    pub fn iter(&self) -> Iter<String, Package> {
        self.all.iter()
    }
}

impl IntoIterator for Packages {
    type Item = (String, Package);

    type IntoIter = IntoIter<String, Package>;

    fn into_iter(self) -> Self::IntoIter {
        self.all.into_iter()
    }
}

impl<'a> Index<&'a str> for Packages {
    type Output = Package;
    fn index(&self, s: &'a str) -> &Self::Output {
        &self.all[s]
    }
}
