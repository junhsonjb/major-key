/**
    A Location struct represents the network address of a node.
    It contains the IP and port of a node, which are used to create
    network (likely TCP) connection.
*/
pub struct Location {
	name: String,
    ip: String,
    port: u16,
}

impl Location {

	/**
		Create a new Location object, this method functions as a constructor.
		Pass in the specified information to create the Location.
	*/
    pub fn new(name: &str, ip: &str, port: u16) -> Location {
        Location { 
			name: String::from(name), 
			ip: String::from(ip), 
			port,
		}
    }

	/**
		Return the name of the instance
	*/
	pub fn get_name(&self) -> &str {
		&self.name
	}

	/**
		Return the IP address of the instance
	*/
    pub fn get_ip(&self) -> &str {
        &self.ip
    }

	/**
		Return the port number of the instance
	*/
    pub fn get_port(&self) -> u16 {
        self.port
    }

	/**
		Return the connection tuple of the instance
	*/
    pub fn get_connection_tuple(&self) -> (&str, u16) {
        (&self.ip, self.port)
    }
}
