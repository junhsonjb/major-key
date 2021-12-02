use major_key::node::Node;
use major_key::node::Rank;
use major_key::librequest;
use major_key::location;
use major_key::value;

use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::fs::File;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::env;
use bson::{Bson, Document};
use std::thread;

// Coped from bin/server.rs -- TODO: find a way to efficiently reuse this code
fn replica_pair(line: &str) -> (String, location::Location) {

	let line_string = line.to_string();
	let words: Vec<&str> = line_string.split_whitespace().collect();

	let name = words[0].to_string();
	let ip = words[1].to_string();
	let port: u16 = words[2].parse().unwrap();
	
	let replica = location::Location::new(&name, &ip, port);

	(name, replica)

}

// Coped from bin/server.rs -- TODO: find a way to efficiently reuse this code
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

	// collect cmd line params
	let args: Vec<String> = env::args().collect();

	if (args.len() != 2) {
		eprintln!("Please try again with the following command structure: ");
		eprintln!("\t./client <node-record-file>");
	}

	let record_name = args[1].to_owned();
	let node_map = gather_nodes(&record_name);
	
	// choose primary (top node in record)
	let primary = node_map.get("replica1").unwrap(); // hardcoded (at least for now)

	match TcpStream::connect(primary.get_connection_tuple()) {

		Ok(mut stream) => {
			println!("client: Successfully connected to primary!");

			println!("client: Doing setup for sending request: hello world");
			// create request to send over network
			let cat = librequest::CRequestType::PUT;
			let stringdata = String::from("hello world");
			// let data = Bson::String(b"hello world".to_string());
			let data = Bson::String(stringdata);
			let val = value::Value::new(data);
			let request_obj = librequest::make_crequest(cat, "test".to_string(), val.as_bytes()).unwrap();
			let request = librequest::serialize_crequest(&request_obj);

			// send over stream
			println!("client: sending request");
			stream.write(&request).expect("issue with writing to stream");

			// wait for response
			const PLACEHOLDER: usize = 500;
			let mut buffer = [0 as u8; PLACEHOLDER];
			println!("client: about to listen for response");
			match stream.read_exact(&mut buffer) {

				Ok(_) => {
					// unpack response
					let response = librequest::deserialize_cresponse(&buffer).unwrap();
					let key = response.key;
					let val = response.value;
					let sts = response.status;
					println!("client: content: k: {}, v: {}, s: {}", key, std::str::from_utf8(&val).unwrap(), sts);
				},
				Err(e) => {
					eprintln!("Failed to recieve data: {}", e);
				},

			}
		},
		Err(e) => {
			eprintln!("Failed to connect: {}", e);
		},

	}

}















