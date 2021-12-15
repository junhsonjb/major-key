use bson::{Bson, Timestamp};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use bincode;

/**
	A wrapper struct for BSON values.
*/
#[derive(Serialize, Deserialize)]
pub struct Value {
    value: Bson,
    timestamp: SystemTime,
}

impl Value {

	/**
		Constructor, pass in a BSON value.
	*/
    pub fn new(val: Bson) -> Value {
        Value {
            value: val,
            timestamp: SystemTime::now(),
        }
    }

	/**
		Return this instance represented in the form of a byte vector.
	*/
	pub fn as_bytes(&self) -> Vec<u8> {
		let bytes = bincode::serialize(&self).unwrap();
		bytes
	}
}
