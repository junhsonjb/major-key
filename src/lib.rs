/*
    Originally, I wrote the library for creating/serializing/deserializing messages
    in this file. I've since decided to move that code into request_lib.rs. I chose to
    this so that this file (lib.rs) can be used for the external library, containing
    functions that users will use to operate the database.

    I don't know for sure that I'll end up needing an external lib file (lib.rs),
    but either way it's still good to have the message code in request_lib.rs because
    the filename is more descriptive.

    So this file is available for use if I need it.
*/
