use tonic::{transport::Server, Request, Response, Status};

pub mod slack {
        tonic::include_proto!("slack"); // proto package name
}

use slack::slack_adaptor_server::{SlackAdaptor, SlackAdaptorServer};
use slack::{PostMessageRequest,PostMessageResponse};

#[derive(Debug, Default)]
pub struct Slack {}

#[tonic::async_trait]
impl SlackAdaptor for Slack {
    async fn post_message(&self, req: Request<PostMessageRequest>) -> Result<Response<PostMessageResponse>, Status> {
        println!("Request: {:?}", req);

        let resp = slack::PostMessageResponse {};

        Ok(Response::new(resp))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let slack = Slack::default();

    Server::builder()
        .add_service(SlackAdaptorServer::new(slack))
        .serve(addr)
        .await?;

    Ok(())
}
