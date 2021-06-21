use tonic::{transport::Server};

pub mod slack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(slack::get_slack())
        .serve(addr)
        .await?;

    Ok(())
}
