#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Fleet {
    pub protocol: String,
    pub provider_version: String,
    pub provider_name: String,
}

impl Fleet {
    pub fn new<S>(protocol: S, provider_version: S, provider_name: S) -> Self
    where
        S: Into<String>,
    {
        Fleet {
            protocol: protocol.into(),
            provider_version: provider_version.into(),
            provider_name: provider_name.into(),
        }
    }
}
