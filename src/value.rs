use bson::{Bson, Timestamp};
use std::time::SystemTime;

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
}
