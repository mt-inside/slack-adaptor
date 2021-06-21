extern crate serde;
extern crate serde_json;

use clap::{AppSettings, Clap};
use tonic::transport::Server;

mod service;
mod client;

#[derive(Clap)]
#[clap(version="0.1", author = "Matt Turner")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long)]
    listen_addr: String,
    #[clap(short, long)]
    channel_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let addr = opts.listen_addr.parse()?;

    let client = client::SlackClient::new(opts.channel_id);

    Server::builder()
        .add_service(service::SlackAdaptorService::new_server(client))
        .serve(addr)
        .await?;

    Ok(())
}
