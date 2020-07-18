mod cli;
mod grpc;
mod web;

use clap::derive::Clap;
use grpc::*;
use web::start_web_server;


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

    let server = tokio::task::spawn(grpc::start_server());

    let client = tokio::task::spawn(async {
        let mut client = create_client().await;
        loop {
            let response = client.say_hello("foo").await;
            println!("{}", response);
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let http_server = tokio::task::spawn(start_web_server());

    server.await?;
    client.await?;
    http_server.await?;

    Ok(())
}
