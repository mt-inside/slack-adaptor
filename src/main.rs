extern crate serde;
extern crate serde_json;

use tonic::transport::Server;

mod service;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let client = client::SlackClient::new();

    Server::builder()
        .add_service(service::SlackAdaptorService::new_server(client))
        .serve(addr)
        .await?;

    Ok(())
}
