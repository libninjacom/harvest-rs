use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveInvoiceMessageSubjectAndBodyForSpecificInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub thank_you: Option<bool>,
    pub reminder: Option<bool>,
}
impl<'a> RetrieveInvoiceMessageSubjectAndBodyForSpecificInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceMessageSubjectAndBody> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/invoices/{invoice_id}/messages/new", invoice_id = self.invoice_id
                ),
            );
        if let Some(ref unwrapped) = self.thank_you {
            r = r.push_query("thank_you", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.reminder {
            r = r.push_query("reminder", &unwrapped.to_string());
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
    pub fn thank_you(mut self, thank_you: bool) -> Self {
        self.thank_you = Some(thank_you);
        self
    }
    pub fn reminder(mut self, reminder: bool) -> Self {
        self.reminder = Some(reminder);
        self
    }
}
