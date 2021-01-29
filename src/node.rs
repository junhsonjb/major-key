use crate::command;
use crate::location;
use crate::value;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::net::{TcpListener, TcpStream, Shutdown};

enum Rank {
    Leader,
    Follower,
    Candidate,
}

pub struct Node {
    name: String,                                      // name (not sure if needed)
    // location: location::Location,                   // location info for this node
    replicas: HashMap<String, location::Location>, // map of the replicas (might need change in structure of map)
    rank: Rank,                         // leader, foller, candidate (primary/secondary)
    // num_replicas: usize,                // the amount of replicas in total
    is_replicated: bool,                // boolean telling if this shard is replicated
    // client: location::Location,     // the location info of the client
    data: HashMap<i32, value::Value>,   // the actual data
    op_log: VecDeque<command::Command>, // holds commands (NEED TO CREATE Command STRUCT
}

impl Node {
    /*
        Create a new object, functions as a constructor. Pass in
        the specified identifying information.
    */
    pub fn new(
        name: &str,
		replicas: HashMap<String, location::Location>,
        // location: location::Location,
        // rank: Rank,
        // num_replicas: usize,
        // is_replicated: bool,
        // client: location::Location,
    ) -> Node {
        Node {
            name: String::from(name),
            // location,
            replicas, // HashMap::new(),
            rank: Rank::Follower,
            // num_replicas: ,
            is_replicated: true,
            // client,
            data: HashMap::new(),
            op_log: VecDeque::new(),
        }
    }

    /*
        Store data on this node. Pass in the key and the value.
        Return and option, either the old value (before the put)
        or None if there was no old value (new key creation).
    */
    pub fn put(&mut self, key: i32, val: value::Value) -> Option<value::Value> {
        self.data.insert(key, val)
    }

    /*
        Retrieve data stored in this node. Pass in the associated key.
        Return an option, either a reference to the corresponding value
        or None if there is no corresponding value.
    */
    pub fn get(&self, key: i32) -> Option<&value::Value> {
        self.data.get(&key)
    }

    /*
        Add a replica to this node's map of replicas. Pass in name and
        Location object. Return an option, either the old value if the
        replica is being updated (not likely) or None if the key is new.
    */
    pub fn add_replica(
        &mut self,
        name: &str,
        location: location::Location,
    ) -> Option<location::Location> {
        self.replicas.insert(String::from(name), location)
    }

    /*
        Remove a replica. Pass in the key (name) of the replica to remove
        from the replicas map. Return an option, either the value that has
        just been removed or None if there was no replica associated with
        the given name.
    */
    pub fn remove_replica(&mut self, name: &str) -> Option<location::Location> {
        self.replicas.remove(name)
    }

	pub fn set_replicas(&mut self, map: HashMap<String, location::Location>) {
		self.replicas = map;
	}

    /*
        Return the amount (as a usize) of replicas that are associated
        with this node, i.e. the other replicas in this shard.
    */
    pub fn get_num_replicas(&self) -> usize {
		self.replicas.len()
    }

    /*
        Return a bool based on whether this node is replicated (true) or
        if it is a single, unreplicated node.
    */
    pub fn is_replicated(&self) -> bool {
        self.is_replicated
    }

    /*
        Set the is_replicated boolean to true or false. Pass in a boolean
        value to set.
    */
    pub fn set_is_replicated(&mut self, replicated: bool) {
        self.is_replicated = replicated;
    }

    /*
        Add a command to the op log. Pass in a command object to
        append to the queue.
    */
    pub fn add_command(&mut self, cmd: command::Command) {
        self.op_log.push_back(cmd);
    }
}

fn replica_pair(line: &str) -> (String, location::Location) {

	let line_string = line.to_string();
	let words: Vec<&str> = line_string.split_whitespace().collect();

	let name = words[0].to_string();
	let ip = words[1].to_string();
	let port: u16 = words[2].parse().unwrap();
	
	let replica = location::Location::new(&name, &ip, port);

	(name, replica)

}

fn gather_nodes(filename: &str) -> HashMap<String, location::Location> {

	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);

	let mut map = HashMap::new();

	for line in reader.lines() {
		let pair = replica_pair(&line.unwrap());
		map.insert(pair.0, pair.1);
	}

	map

}

fn main() {

	let args: Vec<String> = env::args().collect();

	if (args.len() != 2) {
		eprintln!("Please try again with following command:");
		eprintln!("\t./{} <node-name> <node-record-file>", args[0]);
	}

	let node_name = args[1].to_owned();
	let record_name = args[2].to_owned();

	let node_map = gather_nodes(&record_name);
	let home_location = &node_map[&node_name];

	let node = Node::new(&node_name, node_map);

	let listener = TcpListener::bind(node.replicas[&node_name].get_connection_tuple()).unwrap();
	for stream in listener.incoming() {
		match stream {

			Ok(stream) => {
				
			}

			Err(err) => {
				eprintln!("{}", err);
			}

		}
	}

}












