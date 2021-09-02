use bson::{Bson, Timestamp};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use bincode;

#[derive(Serialize, Deserialize)]
pub struct Value {
    value: Bson,
    timestamp: SystemTime,
}

impl Value {
    pub fn new(val: Bson) -> Value {
        Value {
            value: val,
            timestamp: SystemTime::now(),
        }
    }

	pub fn as_bytes(&self) -> Vec<u8> {
		let bytes = bincode::serialize(&self).unwrap();
		bytes
	}
}
