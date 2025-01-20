use tonic::{transport::Server, Request, Response, Status};

use message::message_server::{Message, MessageServer};
use message::{PutRequest, PutResponse, GetRequest, GetResponse};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod message {
    tonic::include_proto!("message");
}

pub struct Node {
    data: Arc<Mutex<HashMap<String, String>>>
}

#[tonic::async_trait]
impl Message for Node {
    async fn put(
        &self,
        request: Request<PutRequest>,
    ) -> Result<Response<PutResponse>, Status> {
        let request = request.into_inner();
        let key = request.key;
        let value = request.value;

        let mut data = self.data.lock().await;
        data.insert(key, value);

        Ok(Response::new(PutResponse{status: true}))
    }

    async fn get(
        &self,
        request: Request<GetRequest>,
    ) -> Result<Response<GetResponse>, Status> {
        let request = request.into_inner();
        let key = request.key;

        let data = self.data.lock().await;
        let result = data.get(&key);

        match result {
            Some(x) => Ok(Response::new(GetResponse{status: true, value: x.to_string()})),
            None    => Ok(Response::new(GetResponse{status: false, value: "".to_string()})),
        }
    }
}

impl Node {
    pub fn new() -> Node {
        Node {data: Arc::new(Mutex::new(HashMap::new()))}
    }

    pub async fn serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = "[::1]:50051".parse()?;
        Server::builder()
            .add_service(MessageServer::new(self))
            .serve(addr)
            .await?;
        Ok(())
    }
}
