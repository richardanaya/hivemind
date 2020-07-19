use crate::grpc::HivemindNodeClient;
use log::*;
use std::net::SocketAddr;

pub enum NodeRequest {
    JoinCluster(Option<SocketAddr>, String),
    AcceptedIntoCluster(Option<SocketAddr>),
    Peers(Option<SocketAddr>),
    NotifyPeers(Option<SocketAddr>),
}

pub async fn start_node(
    channel: flume::Receiver<NodeRequest>,
    mut peer: Option<HivemindNodeClient>,
) {
    loop {
        match channel.try_recv() {
            Ok(r) => {
                info!("received a hello");
            }
            Err(flume::TryRecvError::Empty) => {}
            Err(flume::TryRecvError::Disconnected) => return,
        }
        if let Some(client) = &mut peer {
            println!("sending hello");
            client.say_hello("foo").await;
        }
        tokio::task::yield_now().await;
    }
}
