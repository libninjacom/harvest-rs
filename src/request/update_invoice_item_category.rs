use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_item_category_id: String,
    pub name: Option<String>,
}
impl<'a> UpdateInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceItemCategory> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/invoice_item_categories/{invoice_item_category_id}",
                    invoice_item_category_id = self.invoice_item_category_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
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
}
