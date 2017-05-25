use std::collections::HashMap;

use super::information::*;

pub type InformationCollection = HashMap<String, Information>;


/// Returns a new Information Collection without Packages
pub fn collection_without_packages(collection: InformationCollection) -> InformationCollection {
    let mut new_collection = InformationCollection::with_capacity(collection.len());
    for (host, information) in collection {
        new_collection.insert(host, information.without_packages());
    }

    new_collection.shrink_to_fit();
    new_collection
}

/// Returns a new Information Collection without Packages
pub fn collection_without_packages_ref(collection: &InformationCollection) -> InformationCollection {
    let mut new_collection = InformationCollection::with_capacity(collection.len());
    for (host, information) in collection {
        new_collection.insert(host.clone(), information.without_packages());
    }

    new_collection.shrink_to_fit();
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
