use std::collections::BTreeMap;

use super::information::*;
use crate::error::*;

pub type InformationCollection = BTreeMap<String, Information>;
pub type CollectionResult = Result<(InformationCollection, ErrorCollection), Error>;

trait InformationCollectionTrait {
    fn new_with_capacity(capacity: usize) -> Self;
}

impl InformationCollectionTrait for InformationCollection {
    #[allow(unused_variables)]
    fn new_with_capacity(capacity: usize) -> Self {
        InformationCollection::new()
        //        InformationCollection::with_capacity(collection.len())
    }
}

/// Returns a new Information Collection without Packages
pub fn collection_without_packages(collection: InformationCollection) -> InformationCollection {
    let mut new_collection = InformationCollection::new_with_capacity(collection.len());
    for (host, information) in collection {
        new_collection.insert(host, information.without_packages());
    }

    new_collection
}

/// Returns a new Information Collection without Packages
pub fn collection_without_packages_ref(collection: &InformationCollection) -> InformationCollection {
    let mut new_collection = InformationCollection::new_with_capacity(collection.len());
    for (host, information) in collection {
        new_collection.insert(host.clone(), information.without_packages());
    }

    new_collection
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_test_collection() -> InformationCollection {
        let mut collection = InformationCollection::new();
        collection.insert("localhost".to_owned(), Information::new_for_current_env());
        collection.insert("127.0.0.1".to_owned(), Information::new_for_current_env());

        collection
    }

    fn assert_collection_without_packages(collection: &InformationCollection) {
        for information in collection.values() {
            assert_eq!(0, information.packages.len());
        }
    }

    #[test]
    fn collection_without_packages_ref_test() {
        let collection = build_test_collection();
        let expected_length = collection.len();
        let new_collection = collection_without_packages_ref(&collection);
        assert_eq!(expected_length, new_collection.len());
        assert_collection_without_packages(&new_collection);
    }

    #[test]
    fn collection_without_packages_test() {
        let collection = build_test_collection();
        let expected_length = collection.len();
        let new_collection = collection_without_packages(collection);
        assert_eq!(expected_length, new_collection.len());
        assert_collection_without_packages(&new_collection);
    }
}
