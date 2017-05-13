pub fn map<T, F, U>(collection: &[T], callback: F) -> Vec<U>
    where F: Fn(&T) -> U {
    let mut result = Vec::with_capacity(collection.len());
    for item in collection {
        result.push(callback(item));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let actual = map(
            &vec!(
                 "Apple",
                 "Pear",
                 "Banana",
                 "Orange",
                 ),
            |fruit| fruit.len()
        );

        let expected = vec!(5, 4, 6, 6);

        assert_eq!(expected, actual);
    }

    #[test]
    fn map_to_owned_string_test() {
        let actual = map(
            &vec!(
                 "Apple",
                 "Pear",
                 "Banana",
                 "Orange",
                 ),
            |fruit| String::from(fruit.to_owned())
        );

        let expected = vec!(
                           "Apple",
                           "Pear",
                           "Banana",
                           "Orange",
                           );

        assert_eq!(expected, actual);
    }

    #[test]
    fn map_get_first_item_test() {
        let actual = map(
            &vec!(
                 vec!("Apple", "Red"),
                 vec!("Pear", "Green"),
                 vec!("Banana", "Yellow"),
                 vec!("Orange", "Orange"),
                 ),
            |row| row[0]
        );

        let expected = vec!(
                           "Apple",
                           "Pear",
                           "Banana",
                           "Orange",
                           );

        assert_eq!(expected, actual);
    }

    //    #[test]
    //    fn map_static_string_array_test() {
    //        const FRUITS: &'static [&'static str] = &[
    //            "Apple",
    //            "Pear",
    //            "Banana",
    //            "Orange",
    //        ];
    //
    //        let actual: Vec<String> = map(
    //            FRUITS,
    //            |fruit| fruit.to_owned()
    //        );
    //
    //        let expected = vec!(
    //                           "Apple",
    //                           "Pear",
    //                           "Banana",
    //                           "Orange",
    //                           );
    //
    //        assert_eq!(expected, actual);
    //    }
}
