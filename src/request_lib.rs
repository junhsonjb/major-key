use std::io::Cursor;
use prost::Message;

pub mod request {
	include!(concat!(env!("OUT_DIR"), "/major_key.request.rs"));
}

pub enum CRequestType {
	PUT,
	GET,
}

pub enum NRequestType {
	PUT,
	GET,
	HEARTBEAT,
}

pub fn make_crequest(category: CRequestType, key: String, value: Vec<u8>) -> Option<request::CRequest> {
	let mut req = request::CRequest::default();
	match category {
		CRequestType::PUT => req.set_category(request::CRequestType::Cput),
		CRequestType::GET => req.set_category(request::CRequestType::Cget),
	};
	req.key = key;
	req.value = value;

	Some(req)
}

pub fn make_cresponse(category: CRequestType, key: String, value: Vec<u8>, status: bool) -> Option<request::CResponse> {
	let mut res = request::CResponse::default();
	match category {
		CRequestType::PUT => res.set_category(request::CRequestType::Cput),
		CRequestType::GET => res.set_category(request::CRequestType::Cget),
	};
	res.key = key;
	res.value = value;
	res.status = status;

	Some(res)
}

pub fn make_nrequest(category: NRequestType, key: String, value: Vec<u8>, from: String) -> Option<request::NRequest> {
	let mut req = request::NRequest::default();
	match category {
		NRequestType::PUT => req.set_category(request::NRequestType::Nput),
		NRequestType::GET => req.set_category(request::NRequestType::Nget),
		NRequestType::HEARTBEAT => req.set_category(request::NRequestType::Heartbeat),
	};
	req.key = key;
	req.value = value;
	req.from = from;

	Some(req)
}

pub fn make_nresponse(category: NRequestType, key: String, value: Vec<u8>, from: String, status: bool) -> Option<request::NResponse> {
	let mut res = request::NResponse::default();
	match category {
		NRequestType::PUT => res.set_category(request::NRequestType::Nput),
		NRequestType::GET => res.set_category(request::NRequestType::Nget),
		NRequestType::HEARTBEAT => res.set_category(request::NRequestType::Heartbeat),
	};
	res.key = key;
	res.value = value;
	res.from = from;
	res.status = status;

	Some(res)
}

pub fn serialize_crequest(req: &request::CRequest) -> Vec<u8> {
	let mut buffer = Vec::new();
	buffer.reserve(req.encoded_len());
	req.encode(&mut buffer).unwrap();
	buffer
}

pub fn serialize_cresponse(res: &request::CResponse) -> Vec<u8> {
	let mut buffer = Vec::new();
	buffer.reserve(res.encoded_len());
	res.encode(&mut buffer).unwrap();
	buffer
}


pub fn serialize_nrequest(req: &request::NRequest) -> Vec<u8> {
	let mut buffer = Vec::new();
	buffer.reserve(req.encoded_len());
	req.encode(&mut buffer).unwrap();
	buffer
}

pub fn serialize_nresponse(res: &request::NResponse) -> Vec<u8> {
	let mut buffer = Vec::new();
	buffer.reserve(res.encoded_len());
	res.encode(&mut buffer).unwrap();
	buffer
}


pub fn deserialize_crequest(buffer: &[u8]) -> Result<request::CRequest, prost::DecodeError> {
	request::CRequest::decode(&mut Cursor::new(buffer))
}

pub fn deserialize_cresponse(buffer: &[u8]) -> Result<request::CResponse, prost::DecodeError> {
	request::CResponse::decode(&mut Cursor::new(buffer))
}

pub fn deserialize_nrequest(buffer: &[u8]) -> Result<request::NRequest, prost::DecodeError> {
	request::NRequest::decode(&mut Cursor::new(buffer))
}

pub fn deserialize_nresponse(buffer: &[u8]) -> Result<request::NResponse, prost::DecodeError> {
	request::NResponse::decode(&mut Cursor::new(buffer))
}
