use information::Package;
use information::Packages;
use std::collections::HashMap;

pub struct PackageFilter {}

impl PackageFilter {
    pub fn filter_by_package(packages: Packages, search: &str) -> Packages {
        let all: HashMap<String, Package> = packages.all;
        let filtered: HashMap<String, Package> = all.into_iter().filter(|&(_, ref package)| {
            package.description.contains(search) || package.key.contains(search)
        }).collect();

        Packages::new_with_packages(filtered)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use information::Information;

    #[test]
    fn filter_by_package_test() {
        let result = PackageFilter::filter_by_package(Information::new_for_current_env().packages, "not-a-package");

        assert_eq!(0, result.len());
    }
}
