use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub billable_rate_id: String,
}
impl<'a> RetrieveBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillableRate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/users/{user_id}/billable_rates/{billable_rate_id}", user_id = self
                    .user_id, billable_rate_id = self.billable_rate_id
                ),
            );
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
}
