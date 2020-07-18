use crate::grpc::{HivemindNodeClient, NodeRequest};
use log::*;

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
            let response = client.say_hello("foo").await;
            info!("{}", response);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
}
