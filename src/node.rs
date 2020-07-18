use crate::grpc::create_client;

pub async fn start_node() {
    let mut client = create_client().await;
    loop {
        let response = client.say_hello("foo").await;
        println!("{}", response);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
