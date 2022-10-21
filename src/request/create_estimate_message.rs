use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateEstimateMessageRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
    pub event_type: Option<String>,
    pub recipients: Option<Vec<serde_json::Value>>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub send_me_a_copy: Option<bool>,
}
impl<'a> CreateEstimateMessageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EstimateMessage> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/estimates/{estimate_id}/messages", estimate_id = self.estimate_id
                ),
            );
        if let Some(ref unwrapped) = self.event_type {
            r = r.push_json(json!({ "event_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.recipients {
            r = r.push_json(json!({ "recipients" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subject {
            r = r.push_json(json!({ "subject" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.body {
            r = r.push_json(json!({ "body" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_me_a_copy {
            r = r.push_json(json!({ "send_me_a_copy" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn event_type(mut self, event_type: &str) -> Self {
        self.event_type = Some(event_type.to_owned());
        self
    }
    pub fn recipients(mut self, recipients: Vec<serde_json::Value>) -> Self {
        self.recipients = Some(recipients);
        self
    }
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_owned());
        self
    }
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_owned());
        self
    }
    pub fn send_me_a_copy(mut self, send_me_a_copy: bool) -> Self {
        self.send_me_a_copy = Some(send_me_a_copy);
        self
    }
}
