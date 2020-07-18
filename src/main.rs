mod cli;
mod grpc;
mod node;
mod web;

use clap::derive::Clap;
use log::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("starting");
    let opts = cli::Opts::parse();

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

    let server = tokio::task::spawn(grpc::start_server());
    let client = tokio::task::spawn(node::start_node());
    let http_server = tokio::task::spawn(web::start_web_server());

    server.await?;
    client.await?;
    http_server.await?;

    Ok(())
}
