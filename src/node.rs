use std::collections::HashMap;
use std::collections::VecDeque;
use crate::value;
use crate::location;
use crate::command;

pub enum Rank {
	Leader,
	Follower,
	Candidate,
}

pub struct Node<'a> {

	name: &'a str, // name (not sure if needed)
	location: location::Location<'a>, // location info for this node
	replicas: HashMap<&'a str, location::Location<'a>>, // map of the replicas (might need change in structure of map)
	rank: Rank, // leader, foller, candidate (primary/secondary)
	num_replicas: usize, // the amount of replicas in total
	is_replicated: bool, // boolean telling if this shard is replicated
	client: location::Location<'a>, // the location info of the client
	data: HashMap<i32, value::Value>, // the actual data
	op_log: VecDeque<command::Command>, // holds commands (NEED TO CREATE Command STRUCT

}

impl<'a> Node<'a> {

	/*
		Create a new object, functions as a constructor. Pass in
		the specified identifying information.
	*/
	pub fn new(
		name: &'a str,
		location: location::Location<'a>,
		rank: Rank,
		num_replicas: usize,
		is_replicated: bool,
		client: location::Location<'a>,
		) -> Node<'a> {

		Node {
			name,
			location,
			replicas: HashMap::new(),
			rank,
			num_replicas,
			is_replicated,
			client,
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
	pub fn add_replica(&mut self, name: &'a str, location: location::Location<'a>) -> Option<location::Location<'a>> {
		// am I doing this right?
		self.replicas.insert(name, location)
	}

	/*
		Remove a replica. Pass in the key (name) of the replica to remove
		from the replicas map. Return an option, either the value that has
		just been removed or None if there was no replica associated with
		the given name.
	*/
	pub fn remove_replica(&mut self, name: &'a str) -> Option<location::Location<'a>> {
		self.replicas.remove(name)
	}

	/*
		Return the amount (as a usize) of replicas that are associated
		with this node, i.e. the other replicas in this shard.
	*/
	pub fn get_num_replicas(&self) -> usize {
		self.num_replicas
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






















