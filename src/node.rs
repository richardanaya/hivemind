use crate::grpc::HivemindNodeClient;
use std::net::SocketAddr;

pub enum NodeRequest {
    JoinCluster(Option<SocketAddr>, i32),
    AcceptedIntoCluster(Option<SocketAddr>),
    Peers(Option<SocketAddr>),
    NotifyPeers(Option<SocketAddr>),
}

pub async fn start_node(
    requests_channel: flume::Receiver<NodeRequest>,
    mut peer: Option<HivemindNodeClient>,
    local_node_port: i32,
) {
    let mut node = HivemindNode::new(requests_channel, peer, local_node_port);
    node.run().await;
}

struct HivemindNode {
    peer: Option<HivemindNodeClient>,
    local_node_port: i32,
    requests_channel: flume::Receiver<NodeRequest>,
}

impl HivemindNode {
    fn new(
        requests_channel: flume::Receiver<NodeRequest>,
        mut peer: Option<HivemindNodeClient>,
        local_node_port: i32,
    ) -> HivemindNode {
        HivemindNode {
            peer,
            local_node_port,
            requests_channel,
        }
    }

    async fn run(&mut self) {
        loop {
            match self.requests_channel.try_recv() {
                Ok(r) => {
                    match r {
                        NodeRequest::JoinCluster(source, port) => {
                            // handle request to join cluster ...
                        }
                        NodeRequest::AcceptedIntoCluster(source) => {
                            // handle acceptace into cluster ...
                        }
                        NodeRequest::Peers(source) => {
                            // handle request for my peers ...
                        }
                        NodeRequest::NotifyPeers(source) => {
                            // handle notification of peers ...
                        }
                    }
                }
                Err(flume::TryRecvError::Empty) => {}
                Err(flume::TryRecvError::Disconnected) => return,
            }
            if let Some(client) = &mut self.peer {
                // if we have a peer the user has asked us to join, try to send request to join cluster
                client.join_cluster(self.local_node_port).await;
            }
            tokio::task::yield_now().await;
        }
    }
}
