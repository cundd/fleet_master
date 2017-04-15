use super::*;
//use super::fleet::Fleet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    pub fleet: Fleet,
    pub system: System,
    pub packages: Packages,
}

impl Information {
    //    pub fn new() -> Information {
    //        Information {
    //            fleet: Fleet::new("0.1.0-dummy", "0.1.0-dummy-provider")
    //
    //        }
    //    }
}
