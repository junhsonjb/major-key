use bson::Bson;

mod command;
mod librequest;
mod location;
mod node;
// mod nodewrapper;
mod value;

fn main() {
    println!("Hello, world!");
    println!("{}", env!("OUT_DIR"));
    let data = Bson::String("hello".to_string());
    let val = value::Value::new(data);
}
