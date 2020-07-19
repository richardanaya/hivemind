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
        println!("received hello");
        self.channel.send(NodeRequest::Hello);
        println!("sent to channel hello");
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
    println!("Running hivemind server at http://{}", addr);
    let greeter = HivemindNode { channel };
    Server::builder()
        .add_service(HivemindServer::new(greeter))
        .serve(addr)
        .await
        .unwrap();
}

pub async fn create_client(target: String) -> HivemindNodeClient {
    println!("Attempting to join hivemind node {}", target);
    let c = HivemindClient::connect(target).await.unwrap();
    HivemindNodeClient { grpc_client: c }
}

pub struct HivemindNodeClient {
    grpc_client: HivemindClient<tonic::transport::channel::Channel>,
}

impl HivemindNodeClient {
    pub async fn say_hello(&mut self, t: &str) {
        let request = tonic::Request::new(hivemind::HelloRequest {
            name: t.to_string(),
        });

        self.grpc_client.say_hello(request).await;
        println!("done");
    }

    pub async fn get_key_value(&mut self, _key: &str) -> String {
        "foo".to_string()
    }

    pub async fn set_key_value(&mut self, _key: &str, _value: &str, _value_type: &str) {}
}
