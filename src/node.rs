use crate::command;
use crate::librequest;
use crate::location;
use crate::value;
// use crate::nodewrapper;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;
use bson::{Bson, Document};

#[derive(Copy, Clone)]
enum Rank {
    Leader,
    Follower,
    Candidate,
}

pub struct Node {
    name: String,                                
    replicas: HashMap<String, location::Location>,
    rank: Rank,                         
    is_replicated: bool,                
    data: HashMap<String, value::Value>,   
    op_log: VecDeque<command::Command>, 
}

impl Node {
    /*
        Create a new object, functions as a constructor. Pass in
        the specified identifying information.
    */
    pub fn new(
        name: &str,
		replicas: HashMap<String, location::Location>,
    ) -> Node {
        Node {
            name: String::from(name),
            replicas, 
            rank: Rank::Follower,
            is_replicated: true,
            data: HashMap::new(),
            op_log: VecDeque::new(),
        }
    }

    /*
        Store data on this node. Pass in the key and the value.
        Return an option, either the old value (before the put)
        or None if there was no old value (new key creation).
    */
    pub fn put(&mut self, key: String, val: value::Value) -> Option<value::Value> {
        self.data.insert(key, val)
    }

    /*
        Retrieve data stored in this node. Pass in the associated key.
        Return an option, either a reference to the corresponding value
        or None if there is no corresponding value.
    */
    pub fn get(&self, key: String) -> Option<&value::Value> {
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

fn handle_crequest(buffer: &[u8], node: &mut Node) {

	let message = librequest::deserialize_crequest(buffer).unwrap();

	match node.rank {

		Rank::Leader => {

			match librequest::which_crequest(buffer).unwrap() {

				librequest::CRequestType::PUT => {
					// - store KV pair in data map
					// - send response

					let key = message.key;
					let val = message.value;

					// create Value object from val bytes
					let doc = Document::from_reader(&mut val.as_slice()).unwrap();
					let bson_obj = Bson::from(doc);
					let value = value::Value::new(bson_obj);

					// need access to node object
					node.put(key, value);
				},	

				librequest::CRequestType::GET => {
					// - create resonse with requested data
					// - send response
				},	

			}

		},

		Rank::Follower => {
			// TODO: route message to Leader (`rout_to_leader(...)` or something)
		},

		Rank::Candidate => {
			// For now, candidates will do the same as followers, we may change this later
			// TODO: route message to Leader (`rout_to_leader(...)` or something)
		},

	}
	
}

fn handle_cresponse(buffer: &[u8]) {



}

fn handle_nrequest(buffer: &[u8]) {



}

fn handle_nresponse(buffer: &[u8]) {



}

fn handle_request(mut stream: TcpStream, node: &mut Node) {

	const PLACEHOLDER: usize = 200; // TODO: Figure an upper bound for sizes of
									// serialized messages. Use that once you
									// find it out. Hopefully this value will
									// be enough for now. But using this much
									// may prove to be inefficient in the 
									// long run. So be sure to figure that out
									// when you're able to!
	let mut buffer = [0 as u8; PLACEHOLDER];
	let bytes_read = stream.read_exact(&mut buffer);
	let request_type = librequest::classify(&buffer).unwrap();

	match request_type {
		librequest::RequestType::CREQUEST => handle_crequest(&buffer.to_vec(), node),
		librequest::RequestType::CRESPONSE => handle_cresponse(&buffer.to_vec()),
		librequest::RequestType::NREQUEST => handle_nrequest(&buffer.to_vec()),
		librequest::RequestType::NRESPONSE => handle_nresponse(&buffer.to_vec()),
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

// dummy function for seeing how `move` works with smart pointers
fn dummy_fn(data: Box<i32>) {
	let something = *data;
}


fn main() {

	// NOTE: Maybe send follower/leader status as a cmd line param?

	let args: Vec<String> = env::args().collect();

	if (args.len() != 2) {
		eprintln!("Please try again with following command:");
		eprintln!("\t./{} <node-name> <node-record-file>", args[0]);
	}

	let node_name = args[1].to_owned();
	let record_name = args[2].to_owned();

	let node_map = gather_nodes(&record_name);
	let home_location = &node_map[&node_name];

	let mut node = Node::new(&node_name, node_map);
	// the below line needs to change, use a NodeWrapper constructor
	// let ref mut  nodeptr = node; // using `ref` makes this same as nodewrapper = &node
	// let mut nodewrap = nodewrapper::NodeWrapper::new(nodeptr);
	let node_rank = node.rank;

	let listener = TcpListener::bind(node.replicas[&node_name].get_connection_tuple()).unwrap();
	for stream in listener.incoming() {
		match stream {

			Ok(stream) => {
				handle_request(stream, &mut node);
			}

			Err(err) => {
				eprintln!("{}", err);
			}

		}
	}

}












