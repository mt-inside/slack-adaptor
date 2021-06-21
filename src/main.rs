use tonic::{transport::Server};

mod slack;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(slack::new_server())
        .serve(addr)
        .await?;

    Ok(())
}
