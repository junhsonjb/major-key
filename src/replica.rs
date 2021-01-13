use bson::Bson;
use std::HashMap;

struct Replica {
	count : usize,
	data: HashMap<&str, Bson>,
	ip: &str
	name: &str,
	port: &str,
	// need log -- should it be an object, file, etc??
}

impl Replica {
	fn new(name: &str, ip: &str, port: &str) -> Replica {
		
	}
}
