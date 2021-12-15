use std::io::Cursor;
use prost::Message;

pub mod request {
	include!(concat!(env!("OUT_DIR"), "/major_key.request.rs"));
}

/**
	The different types of CRequests that can be made. CRequests are 
	strictly sent from the client to the a node (specifically, the 
	client can only talk to the primary node).
*/
pub enum CRequestType {
	PUT,
	GET,
	RR, // used to denote that a message needs to be Re-Routed
}

/**
	The different types of NRequests that can be made. NRequests are 
	strictly sent between nodes.
*/
pub enum NRequestType {
	PUT,
	GET,
	HEARTBEAT,
	RR, // used to denote that a message needs to be Re-Routed
}

/**
	The general types of requests that can be made (for times when we need
	to differentiate between Node messages and C messages.
*/
pub enum RequestType {
	CREQUEST,
	CRESPONSE,
	NREQUEST,
	NRESPONSE,
}

/**
	Using the Prost Crate and the defined protobuf file, create a CRequest
	struct, configured using the given params. The created message will be
	wrapped in a request.
*/
pub fn make_crequest(category: CRequestType, key: String, value: Vec<u8>) -> Option<request::CRequest> {
	let mut req = request::CRequest::default();
	match category {
		CRequestType::PUT => req.set_category(request::CRequestType::Cput),
		CRequestType::GET => req.set_category(request::CRequestType::Cget),
		CRequestType::RR => { /* I don't think this should ever happen */ },
	};
	req.key = key;
	req.value = value;

	Some(req)
}

/**
	Using the Prost Crate and the defined protobuf file, create a CResponse
	struct, configured using the given params. The created message will be
	wrapped in a request.
*/
pub fn make_cresponse(category: CRequestType, key: String, value: Vec<u8>, status: bool) -> Option<request::CResponse> {
	let mut res = request::CResponse::default();
	match category {
		CRequestType::PUT => res.set_category(request::CRequestType::Cput),
		CRequestType::GET => res.set_category(request::CRequestType::Cget),
		CRequestType::RR => { /* I don't think this should ever happen */ },
	};
	res.key = key;
	res.value = value;
	res.status = status;

	Some(res)
}

/**
	Using the Prost Crate and the defined protobuf file, create a NRequest
	struct, configured using the given params. The created message will be
	wrapped in a request.
*/
pub fn make_nrequest(category: NRequestType, key: String, value: Vec<u8>, from: String) -> Option<request::NRequest> {
	let mut req = request::NRequest::default();
	match category {
		NRequestType::PUT => req.set_category(request::NRequestType::Nput),
		NRequestType::GET => req.set_category(request::NRequestType::Nget),
		NRequestType::HEARTBEAT => req.set_category(request::NRequestType::Heartbeat),
		NRequestType::RR => { /* I don't think this should ever happen */ },
	};
	req.key = key;
	req.value = value;
	req.from = from;

	Some(req)
}

/**
	Using the Prost Crate and the defined protobuf file, create a NResponse
	struct, configured using the given params. The created message will be
	wrapped in a request.
*/
pub fn make_nresponse(category: NRequestType, key: String, value: Vec<u8>, from: String, status: bool) -> Option<request::NResponse> {
	let mut res = request::NResponse::default();
	match category {
		NRequestType::PUT => res.set_category(request::NRequestType::Nput),
		NRequestType::GET => res.set_category(request::NRequestType::Nget),
		NRequestType::HEARTBEAT => res.set_category(request::NRequestType::Heartbeat),
		NRequestType::RR => { /* I don't think this should ever happen */ },
	};
	res.key = key;
	res.value = value;
	res.from = from;
	res.status = status;

	Some(res)
}

/**
	Using Prost, serialize the given CRequest. Return a vector of bytes.
	In order to differentiate between message types while they are in 
	the form of a byte stream, append CR to denote that this is a CRequest.
*/
pub fn serialize_crequest(req: &request::CRequest) -> Vec<u8> {
	let mut buffer = Vec::new();
	let mut bytes = Vec::new();
	buffer.reserve(req.encoded_len());
	req.encode(&mut buffer).unwrap();
	bytes = b"CR".to_vec();
	bytes.append(&mut buffer);

	bytes
}

/**
	Using Prost, serialize the given CRequest. Return a vector of bytes.
	In order to differentiate between message types while they are in 
	the form of a byte stream, append CA to denote that this is a CResponse
	('A' is for ACK, since Response also starts with an 'R').
*/
pub fn serialize_cresponse(res: &request::CResponse) -> Vec<u8> {
	let mut buffer = Vec::new();
	let mut bytes = Vec::new();
	buffer.reserve(res.encoded_len());
	res.encode(&mut buffer).unwrap();
	bytes = b"CA".to_vec();
	bytes.append(&mut buffer);
	
	bytes
}


/**
	Using Prost, serialize the given CRequest. Return a vector of bytes.
	In order to differentiate between message types while they are in 
	the form of a byte stream, append NR to denote that this is a NRequest.
*/
pub fn serialize_nrequest(req: &request::NRequest) -> Vec<u8> {
	let mut buffer = Vec::new();
	let mut bytes = Vec::new();
	buffer.reserve(req.encoded_len());
	req.encode(&mut buffer).unwrap();
	bytes = b"NR".to_vec();
	bytes.append(&mut buffer);

	bytes
}

/**
	Using Prost, serialize the given CRequest. Return a vector of bytes.
	In order to differentiate between message types while they are in 
	the form of a byte stream, append CR to denote that this is a NResponse
	('A' is for ACK, since Response also starts with an 'R').
*/
pub fn serialize_nresponse(res: &request::NResponse) -> Vec<u8> {
	let mut buffer = Vec::new();
	let mut bytes = Vec::new();
	buffer.reserve(res.encoded_len());
	res.encode(&mut buffer).unwrap();
	bytes = b"NA".to_vec();
	bytes.append(&mut buffer);

	bytes
}

/**
	Using Prost, deserialize a given byte stream into a CRequest.
*/
pub fn deserialize_crequest(buffer: &[u8]) -> Result<request::CRequest, prost::DecodeError> {
	request::CRequest::decode(&mut Cursor::new(buffer[2..].to_vec()))
}

/**
	Using Prost, deserialize a given byte stream into a CResponse.
*/
pub fn deserialize_cresponse(buffer: &[u8]) -> Result<request::CResponse, prost::DecodeError> {
	request::CResponse::decode(&mut Cursor::new(buffer[2..].to_vec()))
}

/**
	Using Prost, deserialize a given byte stream into a NRequest.
*/
pub fn deserialize_nrequest(buffer: &[u8]) -> Result<request::NRequest, prost::DecodeError> {
	request::NRequest::decode(&mut Cursor::new(buffer[2..].to_vec()))
}

/**
	Using Prost, deserialize a given byte stream into a NResponse.
*/
pub fn deserialize_nresponse(buffer: &[u8]) -> Result<request::NResponse, prost::DecodeError> {
	request::NResponse::decode(&mut Cursor::new(buffer[2..].to_vec()))
}

/** 
	Using the RequestType enum, indicate the type of request that is being
	represented by the given byte slice. We can differentiate message types
	using the characters appended to the byte vector during serialization.
*/
pub fn classify(buffer: &[u8]) -> Option<RequestType> {
	match &buffer[0..2] {
		b"CR" => Some(RequestType::CREQUEST),
		b"CA" => Some(RequestType::CRESPONSE),
		b"NR" => Some(RequestType::NREQUEST),
		b"NA" => Some(RequestType::CRESPONSE),
		_ => None,
	}
}

/**
	Using the CRequestType enum, indicate the type of CRequest that is being
	represented by the given byte slice. We can differentiate using the 
	category field of the CRequest struct (which uses a similar internal enum to
	indicate its type).
*/
pub fn which_crequest(buffer: &[u8]) -> Option<CRequestType> {
	let message = deserialize_crequest(buffer).unwrap();
	match message.category {
		0 => Some(CRequestType::PUT),
		1 => Some(CRequestType::GET),
		_ => None,
	}
}

/**
	Using the NRequestType enum, indicate the type of NRequest that is being
	represented by the given byte slice. We can differentiate using the 
	category field of the NRequest struct (which uses a similar internal enum to
	indicate its type).
*/
pub fn which_nrequest(buffer: &[u8]) -> Option<NRequestType> {
	let message = deserialize_nrequest(buffer).unwrap();
	match message.category {
		0 => Some(NRequestType::PUT),
		1 => Some(NRequestType::GET),
		_ => None,
	}
}






