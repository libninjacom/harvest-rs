use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateInvoiceMessageRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub event_type: Option<String>,
    pub recipients: Option<Vec<serde_json::Value>>,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub include_link_to_client_invoice: Option<bool>,
    pub attach_pdf: Option<bool>,
    pub send_me_a_copy: Option<bool>,
    pub thank_you: Option<bool>,
}
impl<'a> CreateInvoiceMessageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceMessage> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/messages", invoice_id = self.invoice_id),
            );
        if let Some(ref unwrapped) = self.event_type {
            r = r.push_json(json!({ "event_type" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.recipients {
            r = r.push_json(json!({ "recipients" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subject {
            r = r.push_json(json!({ "subject" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.body {
            r = r.push_json(json!({ "body" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.include_link_to_client_invoice {
            r = r.push_json(json!({ "include_link_to_client_invoice" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.attach_pdf {
            r = r.push_json(json!({ "attach_pdf" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.send_me_a_copy {
            r = r.push_json(json!({ "send_me_a_copy" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.thank_you {
            r = r.push_json(json!({ "thank_you" : unwrapped }));
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
    pub fn event_type(mut self, event_type: &str) -> Self {
        self.event_type = Some(event_type.to_owned());
        self
    }
    pub fn recipients(mut self, recipients: Vec<serde_json::Value>) -> Self {
        self.recipients = Some(recipients);
        self
    }
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_owned());
        self
    }
    pub fn body(mut self, body: &str) -> Self {
        self.body = Some(body.to_owned());
        self
    }
    pub fn include_link_to_client_invoice(
        mut self,
        include_link_to_client_invoice: bool,
    ) -> Self {
        self.include_link_to_client_invoice = Some(include_link_to_client_invoice);
        self
    }
    pub fn attach_pdf(mut self, attach_pdf: bool) -> Self {
        self.attach_pdf = Some(attach_pdf);
        self
    }
    pub fn send_me_a_copy(mut self, send_me_a_copy: bool) -> Self {
        self.send_me_a_copy = Some(send_me_a_copy);
        self
    }
    pub fn thank_you(mut self, thank_you: bool) -> Self {
        self.thank_you = Some(thank_you);
        self
    }
}
