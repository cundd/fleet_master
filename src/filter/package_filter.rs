use std::collections::HashMap;

use crate::information::Package;
use crate::information::Packages;

pub struct PackageFilter {}

impl PackageFilter {
    /// Search for [`Package`s] matching the given search string
    ///
    /// If `exact` is `TRUE` only the package's key is tested and has to be the same as the search
    /// If `exact` is `FALSE` packages are returned that contain the search string in either the key or description
    pub fn filter(packages: Packages, search: &str, exact: bool) -> Packages {
        let filtered: HashMap<String, Package> = packages
            .into_iter()
            .filter(|&(_, ref package)| {
                if exact {
                    package.key == search
                } else {
                    package.description.contains(search) || package.key.contains(search)
                }
            })
            .collect();

        Packages::new_with_packages(filtered)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helpers::*;

    use super::*;

    #[test]
    fn filter_by_package_empty_test() {
        assert_eq!(
            0,
            PackageFilter::filter(get_test_packages(), "not-a-package", false).len()
        );
        assert_eq!(
            0,
            PackageFilter::filter(get_test_packages(), "not-a-package", true).len()
        );
    }

    #[test]
    fn filter_by_package_test() {
        let result = PackageFilter::filter(get_test_packages(), "news", false);
        assert_eq!(3, result.len());

        let result = PackageFilter::filter(get_test_packages(), "newsletter", false);
        assert_eq!(1, result.len());
        assert!(result.all.get("news").is_none());
        assert!(result.all.get("newsletter").is_some());
        assert_eq!("newsletter", result["newsletter"].key);
    }

    #[test]
    fn filter_by_package_exact_test() {
        let result = PackageFilter::filter(get_test_packages(), "news", true);

        assert_eq!(1, result.len());
    }
}
