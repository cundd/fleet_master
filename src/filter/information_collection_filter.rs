use crate::filter::package_filter::PackageFilter;
use crate::information::InformationCollection;

pub struct InformationCollectionFilter {}

impl InformationCollectionFilter {
    /// Search the collection of [`Information`] instances for [`Packages`] with given search string
    ///
    /// If `exact` is `TRUE` only the package's key is tested and has to be the same as the search
    /// If `exact` is `FALSE` packages are returned that contain the search string in either the key or description
    pub fn filter_by_package(collection: InformationCollection, search: &str, exact: bool) -> InformationCollection {
        collection.into_iter().filter_map(|(host, information)| {
            let packages = information.packages.clone();
            if 0 < PackageFilter::filter(packages, search, exact).len() {
                Some((host, information))
            } else {
                None
            }
        }).collect()
    }
}


#[cfg(test)]
mod tests {
    use crate::information::Information;

    use super::*;

    fn build_test_collection() -> InformationCollection {
        let mut collection = InformationCollection::new();
        collection.insert("localhost".to_owned(), Information::new_for_current_env());
        collection.insert("127.0.0.1".to_owned(), Information::new_for_current_env());

        collection
    }

    #[test]
    fn filter_by_package_test() {
        let collection = build_test_collection();
        let result = InformationCollectionFilter::filter_by_package(collection, "not-a-package", false);

        assert_eq!(0, result.len());
    }
}
