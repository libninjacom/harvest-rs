use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: Option<f64>,
    pub start_date: Option<String>,
}
impl<'a> CreateBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillableRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/users/{user_id}/billable_rates", user_id = self.user_id));
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.start_date {
            r = r.push_json(json!({ "start_date" : unwrapped }));
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
    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }
    pub fn start_date(mut self, start_date: &str) -> Self {
        self.start_date = Some(start_date.to_owned());
        self
    }
}
