use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateTaskRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: Option<String>,
    pub billable_by_default: Option<bool>,
    pub default_hourly_rate: Option<f64>,
    pub is_default: Option<bool>,
    pub is_active: Option<bool>,
}
impl<'a> CreateTaskRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Task> {
        let mut r = self.client.client.post("/tasks");
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billable_by_default {
            r = r.push_json(json!({ "billable_by_default" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default_hourly_rate {
            r = r.push_json(json!({ "default_hourly_rate" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_default {
            r = r.push_json(json!({ "is_default" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn billable_by_default(mut self, billable_by_default: bool) -> Self {
        self.billable_by_default = Some(billable_by_default);
        self
    }
    pub fn default_hourly_rate(mut self, default_hourly_rate: f64) -> Self {
        self.default_hourly_rate = Some(default_hourly_rate);
        self
    }
    pub fn is_default(mut self, is_default: bool) -> Self {
        self.is_default = Some(is_default);
        self
    }
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
}
