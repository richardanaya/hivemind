use clap::derive::Clap;
use hello_world::hivemind_client::*;
use hello_world::hivemind_server::*;
use hello_world::*;
use tonic::{transport::Server, Request, Response, Status};

mod cli;

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

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};

async fn hello(_: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Infallible> {
    Ok(hyper::Response::new(hyper::Body::from("Hello World!")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = cli::Opts::parse();

    println!("Show verbose logs {}", opts.verbose);

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    match opts.subcmd {
        cli::SubCommand::Join(t) => println!(
            "joining {} from {}:{}",
            t.cluster_node_address, t.host, t.port
        ),
        cli::SubCommand::Run(t) => println!("running {}:{}", t.host, t.port),
        cli::SubCommand::Get(t) => println!("getting key {} at {}", t.key, t.cluster_node_address),
        cli::SubCommand::Set(t) => println!(
            "setting key {} to value at {} of type {} at {}",
            t.key, t.value, t.type_of_value, t.cluster_node_address
        ),
    }

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

    let http_server = tokio::task::spawn(async {
        let addr = ([127, 0, 0, 1], 3000).into();
        let server = hyper::Server::bind(&addr).serve(make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(hello))
        }));
        server.await.unwrap();
    });

    server.await?;
    client.await?;
    http_server.await?;

    Ok(())
}
