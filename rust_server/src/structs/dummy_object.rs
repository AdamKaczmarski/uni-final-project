use serde::{Deserialize,Serialize};
use crate::structs::dummy_friend::DummyFriend;
#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DummyObject {
    pub id: String,
    pub index: u8,
    pub guid: String,
    pub is_active: bool,
    pub balance: f32,
    pub picture: String,
    pub age: u8,
    pub eye_color: String,
    pub name: String,
    pub gender: String,
    pub company: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub about: String,
    pub registered: String,
    pub latitude: f64,
    pub longitude: f64,
    pub tags: Vec<String>,
    pub friends: Vec<DummyFriend>,
    pub greeting: String,
    pub favorite_fruit: String,
}
