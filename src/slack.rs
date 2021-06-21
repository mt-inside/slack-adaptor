use tonic::{Request, Response, Status};

mod slackpb {
        tonic::include_proto!("slack"); // proto package name
}

use slackpb::slack_adaptor_server::{SlackAdaptor, SlackAdaptorServer};
use slackpb::{PostMessageRequest,PostMessageResponse};

#[derive(Debug, Default)]
pub struct SlackAdaptorService {}

pub fn new_server () -> SlackAdaptorServer<SlackAdaptorService> {
    let slack = SlackAdaptorService::default();

    SlackAdaptorServer::new(slack)
}

#[tonic::async_trait]
impl SlackAdaptor for SlackAdaptorService {
    async fn post_message(&self, req: Request<PostMessageRequest>) -> Result<Response<PostMessageResponse>, Status> {
        println!("Request: {:?}", req);

        let resp = slackpb::PostMessageResponse {};

        Ok(Response::new(resp))
    }
}
