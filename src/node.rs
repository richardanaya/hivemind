use crate::grpc::create_client;
use log::*;

pub async fn start_node() {
    let mut client = create_client().await;
    loop {
        let response = client.say_hello("foo").await;
        info!("{}", response);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
