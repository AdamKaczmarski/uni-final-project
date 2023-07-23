use serde::{Deserialize,Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Coordinate {
    pub timestamp: u64,
    pub latitude: f64,
    pub longitude: f64,
}
