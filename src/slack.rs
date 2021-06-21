use tonic::{Request, Response, Status};

pub mod slackpb {
        tonic::include_proto!("slack"); // proto package name
}

use slackpb::slack_adaptor_server::{SlackAdaptor, SlackAdaptorServer};
use slackpb::{PostMessageRequest,PostMessageResponse};

#[derive(Debug, Default)]
pub struct Slack {}

pub fn get_slack () -> SlackAdaptorServer<Slack> {
    let slack = Slack::default();

    SlackAdaptorServer::new(slack)
}

#[tonic::async_trait]
impl SlackAdaptor for Slack {
    async fn post_message(&self, req: Request<PostMessageRequest>) -> Result<Response<PostMessageResponse>, Status> {
        println!("Request: {:?}", req);

        let resp = slackpb::PostMessageResponse {};

        Ok(Response::new(resp))
    }
}
