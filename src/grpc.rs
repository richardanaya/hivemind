use hivemind::hivemind_client::*;
use hivemind::hivemind_server::*;
use hivemind::*;
use tonic::{transport::Server, Request, Response, Status};

pub mod hivemind {
    tonic::include_proto!("hivemind");
}

pub struct HivemindNode {
    channel: flume::Sender<NodeRequest>,
}

#[tonic::async_trait]
impl Hivemind for HivemindNode {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        self.channel.send(NodeRequest::Hello);
        Ok(Response::new(HelloReply {}))
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

pub enum NodeRequest {
    Hello,
}

pub async fn start_server(host: String, port: u16, channel: flume::Sender<NodeRequest>) {
    let addr = format!("{}:{}", host, port).parse().unwrap();
    let greeter = HivemindNode { channel };
    Server::builder()
        .add_service(HivemindServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}

pub async fn create_client(target: String) -> HivemindNodeClient {
    let c = HivemindClient::connect(target).await.unwrap();
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
        "goodbye".to_string()
    }

    pub async fn get_key_value(&mut self, _key: &str) -> String {
        "foo".to_string()
    }

    pub async fn set_key_value(&mut self, _key: &str, _value: &str, _value_type: &str) {}
}
