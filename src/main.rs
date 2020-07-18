mod cli;
mod grpc;
mod node;
mod web;

use clap::derive::Clap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let opts = cli::Opts::parse();

    match opts.subcmd {
        cli::SubCommand::Join(_t) => {
            let server = tokio::task::spawn(grpc::start_server());
            let client = tokio::task::spawn(node::start_node());
            let http_server = tokio::task::spawn(web::start_web_server());

            server.await?;
            client.await?;
            http_server.await?;
        }
        cli::SubCommand::Run(_t) => {
            let server = tokio::task::spawn(grpc::start_server());
            let client = tokio::task::spawn(node::start_node());
            let http_server = tokio::task::spawn(web::start_web_server());

            server.await?;
            client.await?;
            http_server.await?;
        }
        cli::SubCommand::Get(t) => {
            let mut client = grpc::create_client().await;
            let key_value = client.get_key_value(&t.key).await;
            println!("{}", key_value);
        }
        cli::SubCommand::Set(t) => {
            let mut client = grpc::create_client().await;
            client.set_key_value(&t.key, &t.value).await
        }
    }

    Ok(())
}
