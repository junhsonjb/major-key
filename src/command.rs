use crate::value;

/**
    The Command struct represents a command that was operated upon
    the database. It contains the key that was created or updated
    and the value it was given.

    Because reads have no effect on the contents of the database,
    a Command will only ever record writes, which are either updates
    of existing entries, creation of new ones, or deletions of entries.

    Commands will be used in the operation logs (op logs), and anywhere
    else that is deemed necessary.
*/
pub struct Command {
    key: u32,
    val: value::Value,
    term: u32, // not sure if I'll use this yet, it's here just in case
}

impl Command {

	/**
		Create a new Command, functions as a constructor. Pass in
		a key, value, and term (used in RAFT consensus algorithm).
	*/
    pub fn new(key: u32, val: value::Value, term: u32) -> Command {
        Command { key, val, term }
    }
}

// TODO: Consider writing a Display trait so commands can be easily printed
//       (and a debug one, maybe)
