use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateCompanyRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub wants_timestamp_timers: Option<bool>,
    pub weekly_capacity: Option<i64>,
}
impl<'a> UpdateCompanyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Company> {
        let mut r = self.client.client.patch("/company");
        if let Some(ref unwrapped) = self.wants_timestamp_timers {
            r = r.push_json(json!({ "wants_timestamp_timers" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.weekly_capacity {
            r = r.push_json(json!({ "weekly_capacity" : unwrapped }));
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
    pub fn wants_timestamp_timers(mut self, wants_timestamp_timers: bool) -> Self {
        self.wants_timestamp_timers = Some(wants_timestamp_timers);
        self
    }
    pub fn weekly_capacity(mut self, weekly_capacity: i64) -> Self {
        self.weekly_capacity = Some(weekly_capacity);
        self
    }
}
