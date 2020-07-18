use hivemind::hivemind_client::*;
use hivemind::hivemind_server::*;
use hivemind::*;
use tonic::{transport::Server, Request, Response, Status};

pub mod hivemind {
    tonic::include_proto!("hivemind");
}

#[derive(Debug, Default)]
pub struct HivemindNode {}

#[tonic::async_trait]
impl Hivemind for HivemindNode {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }

    async fn get_key_value(
        &self,
        _request: Request<GetKeyValueRequest>,
    ) -> Result<Response<GetKeyValueReply>, Status> {
        Ok(Response::new(GetKeyValueReply {
            message: "foo".into(),
        }))
    }

    async fn set_key_value(
        &self,
        _request: Request<SetKeyValueRequest>,
    ) -> Result<Response<SetKeyValueReply>, Status> {
        Ok(Response::new(SetKeyValueReply {
            message: "foo".into(),
        }))
    }
}

pub async fn start_server() {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = HivemindNode::default();
    Server::builder()
        .add_service(HivemindServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}

pub async fn create_client() -> HivemindNodeClient {
    let c = HivemindClient::connect("http://[::1]:50051").await.unwrap();
    HivemindNodeClient { grpc_client: c }
}

pub struct HivemindNodeClient {
    grpc_client: HivemindClient<tonic::transport::channel::Channel>,
}

impl HivemindNodeClient {
    pub async fn say_hello(&mut self, t: &str) -> String {
        let request = tonic::Request::new(hivemind::HelloRequest {
            name: t.to_string(),
        });

        let response = self.grpc_client.say_hello(request).await.unwrap();
        response.get_ref().message.clone()
    }
}
