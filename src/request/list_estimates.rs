use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListEstimatesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: Option<i64>,
    pub updated_since: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub state: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListEstimatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Estimates> {
        let mut r = self.client.client.get("/estimates");
        if let Some(ref unwrapped) = self.client_id {
            r = r.push_query("client_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.updated_since {
            r = r.push_query("updated_since", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.from {
            r = r.push_query("from", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.to {
            r = r.push_query("to", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.state {
            r = r.push_query("state", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.per_page {
            r = r.push_query("per_page", &unwrapped.to_string());
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
    pub fn client_id(mut self, client_id: i64) -> Self {
        self.client_id = Some(client_id);
        self
    }
    pub fn updated_since(mut self, updated_since: &str) -> Self {
        self.updated_since = Some(updated_since.to_owned());
        self
    }
    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_owned());
        self
    }
    pub fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_owned());
        self
    }
    pub fn state(mut self, state: &str) -> Self {
        self.state = Some(state.to_owned());
        self
    }
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = Some(per_page);
        self
    }
}
