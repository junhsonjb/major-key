//! # Major Key
//! 
//! ## What is `Major-Key`?
//! Major-Key (or MK) is an experimental distributed Key-Value (KV) store that can
//! fairly flexibly. It uses strings for keys and BSON objects as value. Using BSON
//! allows values to be any type that is supported by BSON (most needed types are 
//! supported).
//! 
//! ## Why `Major-Key`?
//! MK was inspired by a two main things:
//! 1. A similar, but smaller project completed for CS457 (Topics in Distributed Systems)
//! 2. My time working at MongoDB, working on their NoSQL Distributed data store (Document
//! store, not KV data store)
//! 
//! These experiences helped me realize my interests in databases and distributed systems.
//! So it only felt fitting that thi would be my MS project.
//! 
//! ## Why Rust?
//! For the CS457 project, my team used Python and at MongoDB, I worked mainly in C++.
//! Why, then, did I choose to write this program in Rust? That is an excellent question!
//! Prior to starting this project (actually during the same semester as CS5457), I took
//! CS451: Systems Programming which was being taught in Rust for the first time (by
//! professor Jeremy Blackburn, who is my advisor for this project). After learning Rust 
//! for the class and doing some complext (and extrememly interesting) projects in the 
//! language, I fell in love! While working at MongoDB, I found myself wondering:
//! _what would happen if this database system had been written in Rust instead of C++?_
//! Instead of simply wondering and letting it go, I decided to pursue a similar problem 
//! for my MS project. In the end, I decided to do a KV store instead of a document store
//! because the simplicity of keys and values worked better for the scope of an MS project!
//! 
//! ## Why the name `Major-Key`?
//! This project is my own and in some way more than just writing the code, I wanted to 
//! leave my mark on it. One of the best ways I could think to do this was to give the 
//! project a funny and creative name. Because of this, I chose to name this project
//! `Major-Key`, after the term coined by DJ Khaled coin years ago. Essentially, a 
//! Major Key is something important, a _key_ factor to obtaining success. For example,
//! to get my Master's degree, staying focused and disciplines was a _Major Key_. I love
//! puns, and since my project is a _key_ value store, there was no better option for a
//! name than `Major-Key`!

pub mod node;
pub mod value;
pub mod location;
pub mod command;
pub mod librequest;

pub use node::Node;
pub use node::Rank;
pub use location::Location;
pub use value::Value;
