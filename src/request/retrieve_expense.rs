use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveExpenseRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_id: String,
}
impl<'a> RetrieveExpenseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Expense> {
        let mut r = self
            .client
            .client
            .get(&format!("/expenses/{expense_id}", expense_id = self.expense_id));
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
