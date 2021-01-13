use bson::Bson;

mod value;

fn main() {
	println!("Hello, world!");
	let data = Bson::String("hello".to_string());
	let val = value::Value::new(data);
}
