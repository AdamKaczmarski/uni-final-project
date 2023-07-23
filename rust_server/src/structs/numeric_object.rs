use crate::structs::coordinate::Coordinate;
use serde::{Deserialize, Serialize};
//I have to use &'static str because String is unknown during compilation time
//That means it's put on the heap but stuff that's hardcoded is put on stack
//So rust has to know the amount of bytes to acquire when creating such object
//&str is a slice that's unmutable
//String is expendable (it can be mutated)
#[derive(Clone, Deserialize, Serialize)]
pub struct NumericObject {
    pub _id: String,
    pub index: u8,
    pub numbr: u8,
    pub coordinates: [Coordinate; 10],
}
