use std::path::*;
use error::Error;
use std::env;


pub fn detect_configuration_file() -> Result<PathBuf, Error> {
    let pwd = match env::current_dir() {
        Ok(pwd) => pwd,
        Err(e) => return Err(Error::from_error(e)),
    };

    //    println!("{:?}", env::current_dir());

    let mut dot_file = pwd.clone();
    dot_file.push(".fleet.json");

    if dot_file.exists() {
        return Ok(dot_file);
    }

    let mut file = pwd.clone();
    file.push("fleet.json");

    if file.exists() {
        return Ok(file);
    }

    Err(Error::new(format!("Could not detect the configuration file: No configuration file found in {:?}", pwd)))

    //        if let Ok(pwd) = env::current_dir() {
    //            let mut dot_file = pwd.clone();
    //            dot_file.push(".fleet.json");
    //
    //            if dot_file.exists() {
    //                return Ok(dot_file);
    //            }
    //        }
    //        Err(Error::new("Could not detect current working directory"))
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_helpers;

    // Warning: The tests will only pass if they are not run in parallel
    // RUST_TEST_THREADS=1

    fn is_run_parallel() -> bool {
        if let Ok(test_threads_count_str) = env::var("RUST_TEST_THREADS") {
            return match test_threads_count_str.parse::<i32>() {
                Ok(i) => i > 1,
                Err(_) => false
            }
        }

        return true;
    }

    /// Change the working dir to the given testing directory
    fn set_current_dir_for_testing(path: &str) {
        let new_pwd = test_helpers::get_test_resource_path(path);
        if !new_pwd.exists() {
            panic!("New working dir {:?} does not exist", new_pwd);
        }
        env::set_current_dir(new_pwd.clone()).unwrap();
    }

    #[test]
    fn get_configuration_file_test() {
        if is_run_parallel() {
            println!("Skipping tests based on pwd. Set RUST_TEST_THREADS=1 to run");
            return;
        }

        set_current_dir_for_testing("configuration_file");

        let configuration_file_result = detect_configuration_file();
        assert!(configuration_file_result.is_ok(), "{:?}", configuration_file_result);
        assert_eq!(test_helpers::get_test_resource_path("configuration_file/fleet.json"), configuration_file_result.unwrap());
    }

    #[test]
    fn get_configuration_dot_file_test() {
        if is_run_parallel() {
            println!("Skipping tests based on pwd. Set RUST_TEST_THREADS=1 to run");
            return;
        }

        set_current_dir_for_testing("configuration_dot_file");

        let configuration_file_result = detect_configuration_file();
        assert!(configuration_file_result.is_ok(), "{:?}", configuration_file_result);
        assert_eq!(test_helpers::get_test_resource_path("configuration_dot_file/.fleet.json"), configuration_file_result.unwrap());
    }
}
