use major_key::node::Node;
use major_key::node::Rank;
use major_key::librequest;
use major_key::location;
use major_key::value;
// use crate::librequest;

use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::env;
use bson::{Bson, Document};
use std::thread;

/// Server for the Key-Value store!
/// Uses the Node struct to hold data and handles messages
/// (requests and responses) sent over a TcpStream


fn send_to_leader(buffer: &[u8]) {
	//
}

/// Use the provided TcpStream to send a response back to the client
fn send_cresponse(mut stream: TcpStream, response: Option<librequest::request::CResponse>) {
	// send cresponse
	let resp = response.unwrap();
	let s_response = librequest::serialize_cresponse(&resp);
	stream.write(&s_response).expect("issue with writing in send_cresponse");
}

/// Use the provided TcpStream to send a response back to the requesting node
fn send_nresponse(mut stream: TcpStream, response: Option<librequest::request::NResponse>) {
	let resp = response.unwrap();
	let s_response = librequest::serialize_nresponse(&resp);
	stream.write(&s_response).expect("issue with writing in send_nresponse");
}

/// Given a TcpStream and a buffer containing the recieved byte stream,
/// perform reads/writes on the data (based on what the request says to do).
/// This function is only for requests sent from the client
fn handle_crequest(mut stream: TcpStream, buffer: &[u8], node: &mut Node) {

	println!("server: deserializing request");
	let message = librequest::deserialize_crequest(buffer).unwrap();

	match node.rank {

		Rank::Leader => {

			match librequest::which_crequest(buffer).unwrap() {

				librequest::CRequestType::PUT => {
					// - store KV pair in data map
					// - send response

					println!("server: PUT request");

					let key = message.key;
					let bytes = message.value;

					println!("server: got key and bytes vector");
					// println!("server: {}", key);
					// println!("server: {:#?}", bytes);

					// create Value object from val bytes
					let doc = Document::from_reader(&mut bytes.as_slice()).expect("issue with creating doc");
					let bson_obj = doc.get("x").unwrap();
					// let bson_obj = Bson::from(doc);
					let value = value::Value::new(bson_obj.clone());

					// need access to node object
					node.put(&key, value);

					println!("server: making response");
					let response = librequest::make_cresponse(librequest::CRequestType::PUT, key, bytes, true);

					println!("server: sending response");
					send_cresponse(stream, response);
					println!("server: response sent");
				},	

				librequest::CRequestType::GET => {
					// - retrieve requested data (an option, to cover the event of a failure)
					// - if the option is successful, pass true, else, pass false
					// - create resonse with requested data
					// - send response

					let key = message.key;

					let data = node.get(&key);
					let sts = match data {
						Some(x) => true,
						None	=> false,
					};

					// Need to send back `data` as a slice of bytes ([u8])
					let mut bytes: Vec<u8> = Vec::new();

					match sts {
						true => {
							bytes = data.unwrap().as_bytes();
						},
						false => {
							bytes = message.value;
						},
					}

					let response = librequest::make_cresponse(librequest::CRequestType::GET, key, bytes, sts);
					send_cresponse(stream, response);
				},	

				librequest::CRequestType::RR => {
					// I gotta look around and figure out what I need to do here
				}

			}

		},

		Rank::Follower | Rank::Candidate => {
			// For now, candidates will do the same as followers, we may change this later
			// TODO: route message to Leader (`rout_to_leader(...)` or something)
			// Make a crequest that notifies the caller to re-route request to a Leader
			let key = message.key;
			let bytes = message.value;

			let response = librequest::make_cresponse(librequest::CRequestType::RR, key, bytes, true);
			send_cresponse(stream, response); // I THINK: this should be routed to leader (TODO)
		},

	}
	
}

// NOTE: `cresponse` should never be recieved by nodes, they're only for clients
fn handle_cresponse(buffer: &[u8]) {
	// TODO: figure out what you need to do here
	// NOTE: probably nothing, since cresponse is never recieved by nodes. This
	// function is kinda just a placeholder for the match statement that calls it.
	eprintln!("CResponses should never be sent to Nodes, only Clients!");
}

/// Given a TcpStream and a buffer containing the recieved byte stream,
/// perform reads/writes on the data (based on what the request says to do).
/// This function is only for requests sent from other nodes
fn handle_nrequest(mut stream: TcpStream, buffer: &[u8], node: &mut Node) {

	// NOTE: Remember that an NRequest is a request from another node
	// Upon recieving an NRequest, n:
	// - if n is from a leader, process as normal (update/retrieve and send nresponse)
	// - if n is from a follower:
	//   - if n is a PUT/GET request, ignore it (right? follwers don't get those requests)
	//   - if n is a HEARTBEAT, return true if okay, false if bad (use dummy key/value values)
	//	 - (coming soon) if n is a candidate's vote request, handle as neccessary 
	//     (need to design a scheme for this, like I said, coming soon!)

	let message = librequest::deserialize_nrequest(buffer).unwrap();

	match node.rank {

		Rank::Leader => {

			match librequest::which_nrequest(buffer).unwrap() {

				librequest::NRequestType::PUT => {
					// - store KV pair in data map
					// - send response

					let key = message.key;
					let bytes = message.value;

					// create Value object from val bytes
					let doc = Document::from_reader(&mut bytes.as_slice()).unwrap();
					let bson_obj = Bson::from(doc);
					let value = value::Value::new(bson_obj);

					// need access to node object
					node.put(&key, value);

					// TODO: make and return request
					let from = String::from(node.name.clone());
					let response = librequest::make_nresponse(librequest::NRequestType::PUT, key, bytes, from, true);
					send_nresponse(stream, response);
				},	

				librequest::NRequestType::GET => {
					// - retrieve requested data (an option, to cover the event of a failure)
					// - if the option is successful, pass true, else, pass false
					// - create resonse with requested data
					// - send response

					let key = message.key;

					let data = node.get(&key);
					let sts = match data {
						Some(x) => true,
						None	=> false,
					};

					// Need to send back `data` as a slice of bytes ([u8])
					let mut bytes: Vec<u8> = Vec::new();

					match sts {
						true => {
							bytes = data.unwrap().as_bytes();
						},
						false => {
							bytes = message.value;
						},
					}

					let from = String::from(node.name.clone());
					let response = librequest::make_nresponse(librequest::NRequestType::GET, key, bytes, from, sts);
					send_nresponse(stream, response);
				},	

				librequest::NRequestType::HEARTBEAT => {
					// TODO: figure out what to do here
					// probably send a heartbeat right back, right?
				},

				librequest::NRequestType::RR => {
					// TODO: figure out what to do here
				},

			}

		},

		Rank::Follower | Rank::Candidate => {
			// For now, candidates will do the same as followers, we may change this later
			// TODO: route message to Leader (`rout_to_leader(...)` or something)
			// Make a crequest that notifies the caller to re-route request to a Leader
			let key = message.key;
			let bytes = message.value;

			let from = String::from(node.name.clone());
			let response = librequest::make_nresponse(librequest::NRequestType::RR, key, bytes, from, true);
			send_nresponse(stream, response);
		},

	}

}

fn handle_nresponse(stream: TcpStream, buffer: &[u8]) {
	// TODO: figure out what to do here
	// these are just acks, so don't we just read them and do nothing?
}

/// General request handler for all requests. This function will classify a request
/// and send the TcpStream, Node, and buffer to the proper handler (CRequest or 
/// NRequest handler).
fn handle_request(mut stream: TcpStream, node: &mut Node) {

	const PLACEHOLDER: usize = 500; // TODO: Figure an upper bound for sizes of
									// serialized messages. Use that once you
									// find it out. Hopefully this value will
									// be enough for now. But using this much
									// may prove to be inefficient in the 
									// long run. So be sure to figure that out
									// when you're able to!
	let mut pbuffer = [0 as u8; PLACEHOLDER]; // pbuffer is placeholder buffer
	let bytes_read = stream.read(&mut pbuffer).expect("issue with reading stream"); // used to be read_exact(...)
	let buffer = &pbuffer[0..bytes_read];
	let request_type = librequest::classify(&buffer).unwrap();

	// NOTE: remember that if the return type is a message with type
	// `librequest::CRequestType::RR`, then the message needs to be re-routed
	// to a leader instead of a follower or candidate
	println!("server: handle request based on type");
	match request_type {
		librequest::RequestType::CREQUEST => handle_crequest(stream, &buffer.to_vec(), node),
		librequest::RequestType::CRESPONSE => handle_cresponse(&buffer.to_vec()), // nodes don't recieve these
		librequest::RequestType::NREQUEST => handle_nrequest(stream, &buffer.to_vec(), node),
		librequest::RequestType::NRESPONSE => handle_nresponse(stream, &buffer.to_vec()),
	};

}

/// Helper function for gather_nodes(...), reads a line from the file and 
/// parses it into a Location struct instance (to be the value in a HashMap
/// of type String -> location::Location)
fn replica_pair(line: &str) -> (String, location::Location) {

	let line_string = line.to_string();
	let words: Vec<&str> = line_string.split_whitespace().collect();

	let name = words[0].to_string();
	let ip = words[1].to_string();
	let port: u16 = words[2].parse().unwrap();
	
	let replica = location::Location::new(&name, &ip, port);

	(name, replica)

}

/// Create a HashMap of String -> location::Location from a given file
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

	/// Collect command line parameters
	let args: Vec<String> = env::args().collect();

	/// Ensure binary is being called correctly
	if (args.len() != 3) {
		eprintln!("Please try again with following command:");
		eprintln!("\t./{} <node-name> <node-record-file>", args[0]);
	}

	// TODO: each line in node map should *also* denote leader/follower
	// Then, we need to define leader/follower based on this

	let node_name = args[1].to_owned();
	let record_name = args[2].to_owned();

	let node_map = gather_nodes(&record_name);
	let home_location = &node_map[&node_name];

	let mut node = Node::new(&node_name, node_map);
	// the below line needs to change, use a NodeWrapper constructor
	// let ref mut  nodeptr = node; // using `ref` makes this same as nodewrapper = &node
	// let mut nodewrap = nodewrapper::NodeWrapper::new(nodeptr);
	node.rank = Rank::Leader;
	let node_rank = node.rank;

	/// Start listening for requests
	let listener = TcpListener::bind(node.replicas[&node_name].get_connection_tuple()).unwrap();
	for stream in listener.incoming() {
		match stream {

			Ok(stream) => {
				/// Handle the recieved request
				println!("server: about to handle a request!");
				handle_request(stream, &mut node);
			}

			Err(err) => {
				eprintln!("{}", err);
			}

		}
	}

}
