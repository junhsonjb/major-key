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
use bson::{doc, Bson, Document};
use std::thread;

/// A simple client to test/demonstrate server (and key-value store overall)

/// Helper function for gather_nodes(...), reads a line from the file and 
/// parses it into a Location struct instance (to be the value in a HashMap
/// of type String -> location::Location)
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

/// Create a HashMap of String -> location::Location from a given file
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

	/// Collect command line parameters
	// collect cmd line params
	let args: Vec<String> = env::args().collect();

	/// Ensure that the binary was called correctly
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
			/// Create the request that we'll send over the network
			// create request to send over network
			let cat = librequest::CRequestType::PUT;
			let stringdata = String::from("hello world");
			let data = Bson::String(stringdata);
			// let val = value::Value::new(data);
			let doc = doc!{"x": data};
			let mut byte_stream: Vec<u8> = Vec::new();
			doc.to_writer(&mut byte_stream).unwrap();

			/// Put request data into request object and serialize it
			let request_obj = librequest::make_crequest(cat, "test".to_string(), byte_stream).unwrap();
			let request = librequest::serialize_crequest(&request_obj);

			println!("client: MSG LEN - {} bytes", request.len());

			/// Send request byte stream over network
			// send over stream
			println!("client: sending request");
			stream.write(&request).expect("issue with writing to stream");

			// wait for response
			const PLACEHOLDER: usize = 500;
			// let mut pbuffer = [0 as u8; PLACEHOLDER];
			let mut pbuffer = Vec::new();
			println!("client: about to listen for response");
			// TODO: try read_to_end(...)
			match stream.read_to_end(&mut pbuffer) {

				Ok(bytes_read) => {
					println!("client: unpacking response");
					// unpack response

					let buffer = &pbuffer[0..bytes_read];

					// println!("client: {:?}", buffer);
					// println!("client: {:?}", pbuffer);
					println!("client: RESPONSE LEN - {}", buffer.len());
					println!("client: RESPONSE - {:?}", buffer);
					/// Deserialize response (sent from server to indicate results and status of request)
					let response = librequest::deserialize_cresponse(&buffer).unwrap();
					let key = response.key;
					let bytes = response.value;
					let sts = response.status;

					/// Turn response into document (to convert from bytes to BSON)
					let doc = Document::from_reader(&mut bytes.as_slice()).expect("in client, creating document");
					let bson_obj = doc.get("x").unwrap();

					// println!("client: content: k: {}, v: {}, s: {}", key, std::str::from_utf8(&val).unwrap(), sts);
					println!("client: content: k: {}, v: {}, s: {}", key, bson_obj, sts);
				},
				Err(e) => {
					eprintln!("client: Failed to recieve data: {}", e);
				},

			} //
		},
		Err(e) => {
			eprintln!("Failed to connect: {}", e);
		},

	}

}















