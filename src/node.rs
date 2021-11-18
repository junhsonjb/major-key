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

/* This is here so that we can RETURN enttites from this class ONLY.
   Please do not create entities from this class, there is a library
   made specifically for that purpose. Use it!
*/
/* I think this is causing issues bc it's treated as a separate entity from 
   the inclusion written up in librequest.rs
pub mod request {
	include!(concat!(env!("OUT_DIR"), "/major_key.request.rs"));
}
*/

#[derive(Copy, Clone)]
pub enum Rank {
    Leader,
    Follower,
    Candidate,
}

pub struct Node {
    pub name: String,                                
    pub replicas: HashMap<String, location::Location>,
    pub rank: Rank,                         
    pub is_replicated: bool,                
    pub data: HashMap<String, value::Value>,   
    pub op_log: VecDeque<command::Command>, 
}

impl Node {
    /**
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

    /**
        Store data on this node. Pass in the key and the value.
        Return an option, either the old value (before the put)
        or None if there was no old value (new key creation).
    */
    pub fn put(&mut self, key: &str, val: value::Value) -> Option<value::Value> {
        self.data.insert(String::from(key), val)
    }

    /**
        Retrieve data stored in this node. Pass in the associated key.
        Return an option, either a reference to the corresponding value
        or None if there is no corresponding value.
    */
    pub fn get(&self, key: &str) -> Option<&value::Value> {
        self.data.get(key)
    }

    /**
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

    /**
        Remove a replica. Pass in the key (name) of the replica to remove
        from the replicas map. Return an option, either the value that has
        just been removed or None if there was no replica associated with
        the given name.
    */
    pub fn remove_replica(&mut self, name: &str) -> Option<location::Location> {
        self.replicas.remove(name)
    }

    /**
        Return the amount (as a usize) of replicas that are associated
        with this node, i.e. the other replicas in this shard.
    */
    pub fn get_num_replicas(&self) -> usize {
		self.replicas.len()
    }

    /**
        Return a bool based on whether this node is replicated (true) or
        if it is a single, unreplicated node.
    */
    pub fn is_replicated(&self) -> bool {
        self.is_replicated
    }

    /**
        Set the is_replicated boolean to true or false. Pass in a boolean
        value to set.
    */
    pub fn set_is_replicated(&mut self, replicated: bool) {
        self.is_replicated = replicated;
    }

    /**
        Add a command to the op log. Pass in a command object to
        append to the queue.
    */
    pub fn add_command(&mut self, cmd: command::Command) {
        self.op_log.push_back(cmd);
    }

}









