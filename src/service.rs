use tonic::{Request, Response, Status};

use crate::client;

mod slackpb {
    tonic::include_proto!("slack"); // proto package name
}

use slackpb::slack_adaptor_server::{SlackAdaptor, SlackAdaptorServer};
use slackpb::{PostMessageRequest, PostMessageResponse};

#[derive(Debug)]
pub struct SlackAdaptorService {
    client: client::SlackClient,
}

impl SlackAdaptorService {
    pub fn new_server(client: client::SlackClient) -> SlackAdaptorServer<SlackAdaptorService> {
        let slack = SlackAdaptorService { client: client };

        SlackAdaptorServer::new(slack)
    }
}

#[tonic::async_trait]
impl SlackAdaptor for SlackAdaptorService {
    async fn post_message(
        &self,
        req: Request<PostMessageRequest>,
    ) -> Result<Response<PostMessageResponse>, Status> {
        //println!("Request: {:?}", req);

        self.client.slack_post(&req.into_inner().message).await.expect("couldn't post");

        let resp = slackpb::PostMessageResponse {};

        Ok(Response::new(resp))
    }
}
