mod fleet;
mod system;
mod platform;
mod application;
mod packages;
mod information;

use std::collections::HashMap;
pub use self::fleet::*;
pub use self::system::*;
pub use self::packages::*;
pub use self::information::Information;
pub use self::platform::Platform;

pub type InformationCollection = HashMap<String, Information>;
