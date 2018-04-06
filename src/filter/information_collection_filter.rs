use information::InformationCollection;

pub struct InformationCollectionFilter {}

impl InformationCollectionFilter {
    pub fn filter_by_package(collection: InformationCollection, search: &str) -> InformationCollection {
        collection.into_iter().filter_map(|(host, information)| {
            let found_package = information.packages.all.iter().find(|&(_, package)| {
                package.description.contains(search) || package.key.contains(search)
            });
            if found_package.is_none() {
                None
            } else {
                Some((host, information.clone()))
            }
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use information::Information;

    fn build_test_collection() -> InformationCollection {
        let mut collection = InformationCollection::new();
        collection.insert("localhost".to_owned(), Information::new_for_current_env());
        collection.insert("127.0.0.1".to_owned(), Information::new_for_current_env());

        collection
    }

    #[test]
    fn filter_by_package_test() {
        let collection = build_test_collection();

        let result = InformationCollectionFilter::filter_by_package(collection, "not-a-package");

        assert_eq!(0, result.len());
    }
}
