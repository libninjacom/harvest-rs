use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveExpenseCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_category_id: String,
}
impl<'a> RetrieveExpenseCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseCategory> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/expense_categories/{expense_category_id}", expense_category_id =
                    self.expense_category_id
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
