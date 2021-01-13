use bson::{Bson, Timestamp};
use std::collections::HashMap;

struct Server {
	count: usize,
	name: &str,
	replica_map: HashMap<String, replica::Replica>,
}

impl Server {
	pub fn new(name: &str, count: i32) -> Server {
		Server {
			name,
			replica_map: HashMap::with_capacity(count),
		}
	}

	pub fn put(key: &str, value: Bson) {
		// put data in all replicas
	}

	pub fn get(key: &str) -> Bson {
		// get data from primary replica
		// (maybe a secondary if we're allowing that)
	}
}
