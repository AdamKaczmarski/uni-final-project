use serde::{Deserialize,Serialize};
#[derive(Deserialize, Serialize)]
pub struct DummyFriend {
    pub id: u8,
    pub name: String,
}
