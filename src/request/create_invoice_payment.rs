use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateInvoicePaymentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub amount: Option<f64>,
    pub paid_at: Option<String>,
    pub paid_date: Option<String>,
    pub notes: Option<String>,
}
impl<'a> CreateInvoicePaymentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoicePayment> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/payments", invoice_id = self.invoice_id),
            );
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.paid_at {
            r = r.push_json(json!({ "paid_at" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.paid_date {
            r = r.push_json(json!({ "paid_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notes {
            r = r.push_json(json!({ "notes" : unwrapped }));
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
    pub fn paid_at(mut self, paid_at: &str) -> Self {
        self.paid_at = Some(paid_at.to_owned());
        self
    }
    pub fn paid_date(mut self, paid_date: &str) -> Self {
        self.paid_date = Some(paid_date.to_owned());
        self
    }
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_owned());
        self
    }
}
