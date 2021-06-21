use serde::Serialize;

#[derive(Debug)]
pub struct SlackClient {}

#[derive(Debug, Serialize)]
struct PostMessage {
    channel: String,
    text: String,
}

impl SlackClient {
    pub fn new() -> Self {
        SlackClient {}
    }

    pub async fn slack_post(&self) -> Result<(), reqwest::Error> {
        let msg = PostMessage {
            channel: "TODO".into(),
            text: "Hello from gateway".into(),
        };

        let res : serde_json::Value = reqwest::Client::new()
            .post("https://slack.com/api/chat.postMessage")
            .json(&msg)
            .send()
            .await?
            .json()
            .await?;
        println!("{:#?}", res);

        Ok(())
    }
}
