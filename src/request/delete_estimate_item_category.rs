use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteEstimateItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_item_category_id: String,
}
impl<'a> DeleteEstimateItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/estimate_item_categories/{estimate_item_category_id}",
                    estimate_item_category_id = self.estimate_item_category_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
