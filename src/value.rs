// The "value" componet in the key-value pair, contains a value of any type and a timestamp

use bson::{Bson, Timestamp};
use std::time::SystemTime;

struct Value {
	value: Bson,
	timestamp: SystemTime,
}

impl Value {
	pub fn new(&Bson val) -> Value {
		Value {
			value: val,
			timestamp: SystemTime::now()
		}
	}
}
