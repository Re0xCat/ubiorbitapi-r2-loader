use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub orbit: Orbit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Orbit {
    pub name: String,
    pub product_id: u32,
    pub saves: String,
    pub cd_keys: Vec<String>,
    pub log: Log,
    pub profile: Profile,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Log {
    pub write: bool,
    pub path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Profile {
    pub account_id: String,
    pub username: String,
    pub password: String,
}
