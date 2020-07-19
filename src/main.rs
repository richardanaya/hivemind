mod cli;
mod grpc;
mod node;
mod web;

use clap::derive::Clap;
use tokio::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let opts = cli::Opts::parse();

    match opts.subcmd {
        cli::SubCommand::Join(cmd) => {
            let (tx, rx) = flume::unbounded::<grpc::NodeRequest>();
            let server = tokio::task::spawn(grpc::start_server(cmd.host, cmd.port, tx));
            let peer = grpc::create_client(cmd.cluster_node_address).await;
            let client = tokio::task::spawn(node::start_node(rx, Some(peer)));
            let http_server = tokio::task::spawn(web::start_web_server(cmd.web_monitor));
            try_join!(server, http_server, client);
        }
        cli::SubCommand::Run(cmd) => {
            let (tx, rx) = flume::unbounded::<grpc::NodeRequest>();
            let server = tokio::task::spawn(grpc::start_server(cmd.host, cmd.port, tx));
            let client = tokio::task::spawn(node::start_node(rx, None));
            let http_server = tokio::task::spawn(web::start_web_server(cmd.web_monitor));
            try_join!(server, http_server, client);
        }
        cli::SubCommand::Get(cmd) => {
            let mut client = grpc::create_client(cmd.cluster_node_address).await;
            let key_value = client.get_key_value(&cmd.key).await;
            println!("{}", key_value);
        }
        cli::SubCommand::Set(cmd) => {
            let mut client = grpc::create_client(cmd.cluster_node_address).await;
            client
                .set_key_value(&cmd.key, &cmd.value, &cmd.value_type)
                .await
        }
    }

    Ok(())
}
