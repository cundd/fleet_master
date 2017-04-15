#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Fleet {
    pub protocol: String,
    pub provider_version: String,
}

//impl Fleet {
//    pub fn new<S>(protocol: S, provider_version: S) -> Fleet where S: Into<String> {
//        Fleet {
//            protocol: protocol.into(),
//            provider_version: provider_version.into()
//        }
//    }
//}

// "fleet": {
//        "protocol": "0.1.0",
//        "providerVersion": "0.1.0"
//    },