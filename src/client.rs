use serde::Serialize;

// const app_id: &str = "A025QUX08GK";
// const client_id: &str = "1027130396499.2194983008563";
// const client_secret: &str = "07e10345a828fc0a6b4e2820a60e6eef";
// const signing_secret: &str = "c8d8b0fd7cced5947a39de7ee882152c";
const bot_user_oauth_token: &str = "xoxb-1027130396499-2218488047280-fXeNHGtDo9gcs5hXxevJt6C4"; // TODO: read from k8s via fs

#[derive(Debug)]
pub struct SlackClient {
    channel_id: String,
}

#[derive(Debug, Serialize)]
struct PostMessage<'a> {
    channel: String, // TODO &str? idk enough about lifetimes to make this work
    text: &'a str,
}

impl SlackClient {
    pub fn new(channel_id: String) -> Self {
        SlackClient {channel_id: channel_id}
    }

    pub async fn slack_post(&self, msg: &str) -> Result<(), reqwest::Error> {
        let msg = PostMessage {
            channel: self.channel_id.clone(),
            text: msg,
        };

        let res : serde_json::Value = reqwest::Client::new()
            .post("https://slack.com/api/chat.postMessage")
            .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + bot_user_oauth_token)
            .json(&msg)
            .send()
            .await?
            .json()
            .await?;
        //println!("{:#?}", res);
        if *res.get("ok").unwrap() != serde_json::json!(true) {
            panic!("post didn't work"); // TODO: how to return error from functino?
        }

        Ok(())
    }
}
