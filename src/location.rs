/*
	A Location struct represents the network address of a node.
	It contains the IP and port of a node, which are used to create
	network (likely TCP) connection.
*/

pub struct Location<'a> {
	
	ip: &'a str,
	port: u32,

}

impl<'a> Location<'a> {

	pub fn new(ip: &str, port: u32) -> Location {
		Location {
			ip,
			port,
		}
	}
	
	pub fn get_ip(&self) -> &str {
		self.ip
	}

	pub fn get_port(&self) -> u32 {
		self.port
	}

	pub fn get_connection_tuple(&self) -> (&str, u32) {
		(self.ip, self.port)
	}

}
