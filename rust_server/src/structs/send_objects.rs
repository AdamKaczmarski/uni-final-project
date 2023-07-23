use serde::Deserialize;
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SendObjects {
    pub amount: u32,
    pub provider_url: String,
    pub object_format: String,
    pub server_type: String,
    pub object_type: String
}
