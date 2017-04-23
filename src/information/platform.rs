use std::process::Command;

fn uname(key: &str) -> String {
    let flag = format!("-{}", key);
    let output = match Command::new("uname").arg(flag).output() {
        Ok(o) => o,
        Err(e) => panic!(e),
    };
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    String::from(stdout.trim())
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Os {
    pub vendor: String,
    pub version: String,
    pub machine: String,
    pub info: String,
}

impl Os {
    fn new_for_current_env() -> Self {
        Os {
            vendor: uname("s").to_owned(),
            version: uname("r").to_owned(),
            machine: uname("m").to_owned(),
            info: uname("v").to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Platform {
    pub language: String,
    pub version: String,
    pub sapi: String,
    pub host: String,
    pub os: Os,
}

impl Platform {
    pub fn new_for_current_env() -> Self {
        Platform {
            language: "rust".to_owned(),
            version: "x.y.z".to_owned(),
            sapi: "".to_owned(),
            host: uname("h").to_owned(),
            os: Os::new_for_current_env(),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_for_current_env_test() {
        let platform = Platform::new_for_current_env();

        assert_eq!("rust", platform.language);
        assert_eq!("", platform.sapi);
    }
}
