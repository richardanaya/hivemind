use tonic::{transport::Server, Request, Response, Status};

use hello_world::hivemind_client::*;
use hello_world::hivemind_server::*;
use hello_world::*;

pub mod hello_world {
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
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn get_key_value(
        &self,
        _request: Request<GetKeyValueRequest>,
    ) -> Result<Response<GetKeyValueReply>, Status> {
        Ok(Response::new(hello_world::GetKeyValueReply {
            message: "foo".into(),
        }))
    }

    async fn set_key_value(
        &self,
        _request: Request<SetKeyValueRequest>,
    ) -> Result<Response<SetKeyValueReply>, Status> {
        Ok(Response::new(hello_world::SetKeyValueReply {
            message: "foo".into(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = tokio::task::spawn(async {
        let addr = "[::1]:50051".parse().unwrap();
        let greeter = HivemindNode::default();
        Server::builder()
            .add_service(HivemindServer::new(greeter))
            .serve(addr)
            .await
            .unwrap();
    });

    let client = tokio::task::spawn(async {
        loop {
            let mut client = HivemindClient::connect("http://[::1]:50051").await.unwrap();

            let request = tonic::Request::new(HelloRequest {
                name: "Tonic".into(),
            });

            let response = client.say_hello(request).await.unwrap();

            println!("RESPONSE={:?}", response);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    server.await?;
    client.await?;

    Ok(())
}
