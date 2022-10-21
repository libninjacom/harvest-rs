use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveCostRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub cost_rate_id: String,
}
impl<'a> RetrieveCostRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CostRate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/users/{user_id}/cost_rates/{cost_rate_id}", user_id = self.user_id,
                    cost_rate_id = self.cost_rate_id
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
