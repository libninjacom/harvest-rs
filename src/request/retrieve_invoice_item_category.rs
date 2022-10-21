use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_item_category_id: String,
}
impl<'a> RetrieveInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceItemCategory> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/invoice_item_categories/{invoice_item_category_id}",
                    invoice_item_category_id = self.invoice_item_category_id
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
