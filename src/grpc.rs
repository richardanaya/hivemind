use crate::node::NodeRequest;
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
    async fn get_key_value(
        &self,
        _req: Request<GetKeyValueRequest>,
    ) -> Result<Response<GetKeyValueReply>, Status> {
        Ok(Response::new(GetKeyValueReply {
            message: "foo".into(),
        }))
    }

    async fn set_key_value(
        &self,
        _req: Request<SetKeyValueRequest>,
    ) -> Result<Response<SetKeyValueReply>, Status> {
        Ok(Response::new(SetKeyValueReply {
            message: "foo".into(),
        }))
    }

    async fn join_cluster(
        &self,
        req: Request<JoinClusterRequest>,
    ) -> Result<Response<Empty>, Status> {
        self.channel.send(NodeRequest::JoinCluster(
            req.remote_addr(),
            req.get_ref().port.clone(),
        ));
        Ok(Response::new(Empty {}))
    }

    async fn accepted_into_cluster(&self, req: Request<Empty>) -> Result<Response<Empty>, Status> {
        self.channel
            .send(NodeRequest::AcceptedIntoCluster(req.remote_addr()));
        Ok(Response::new(Empty {}))
    }

    async fn get_peers(&self, req: Request<Empty>) -> Result<Response<Empty>, Status> {
        self.channel.send(NodeRequest::Peers(req.remote_addr()));
        Ok(Response::new(Empty {}))
    }

    async fn notify_peers(&self, req: Request<PeersReply>) -> Result<Response<Empty>, Status> {
        self.channel
            .send(NodeRequest::NotifyPeers(req.remote_addr()));
        Ok(Response::new(Empty {}))
    }
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
    pub async fn join_cluster(&mut self, port: &str) {
        let request = tonic::Request::new(hivemind::JoinClusterRequest {
            port: port.to_string(),
        });
        self.grpc_client.join_cluster(request).await;
    }

    pub async fn get_peers(&mut self, port: &str) {
        let request = tonic::Request::new(hivemind::Empty {});
        self.grpc_client.get_peers(request).await;
    }

    pub async fn get_key_value(&mut self, _key: &str) -> String {
        "foo".to_string()
    }

    pub async fn set_key_value(&mut self, _key: &str, _value: &str, _value_type: &str) {}
}
