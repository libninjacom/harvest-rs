use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
    pub client_id: Option<i64>,
    pub number: Option<String>,
    pub purchase_order: Option<String>,
    pub tax: Option<f64>,
    pub tax2: Option<f64>,
    pub discount: Option<f64>,
    pub subject: Option<String>,
    pub notes: Option<String>,
    pub currency: Option<String>,
    pub issue_date: Option<String>,
    pub line_items: Option<Vec<serde_json::Value>>,
}
impl<'a> UpdateEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Estimate> {
        let mut r = self
            .client
            .client
            .patch(&format!("/estimates/{estimate_id}", estimate_id = self.estimate_id));
        if let Some(ref unwrapped) = self.client_id {
            r = r.push_json(json!({ "client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.number {
            r = r.push_json(json!({ "number" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.purchase_order {
            r = r.push_json(json!({ "purchase_order" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax {
            r = r.push_json(json!({ "tax" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.tax2 {
            r = r.push_json(json!({ "tax2" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.discount {
            r = r.push_json(json!({ "discount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.subject {
            r = r.push_json(json!({ "subject" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notes {
            r = r.push_json(json!({ "notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_json(json!({ "currency" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.issue_date {
            r = r.push_json(json!({ "issue_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.line_items {
            r = r.push_json(json!({ "line_items" : unwrapped }));
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
    pub fn client_id(mut self, client_id: i64) -> Self {
        self.client_id = Some(client_id);
        self
    }
    pub fn number(mut self, number: &str) -> Self {
        self.number = Some(number.to_owned());
        self
    }
    pub fn purchase_order(mut self, purchase_order: &str) -> Self {
        self.purchase_order = Some(purchase_order.to_owned());
        self
    }
    pub fn tax(mut self, tax: f64) -> Self {
        self.tax = Some(tax);
        self
    }
    pub fn tax2(mut self, tax2: f64) -> Self {
        self.tax2 = Some(tax2);
        self
    }
    pub fn discount(mut self, discount: f64) -> Self {
        self.discount = Some(discount);
        self
    }
    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_owned());
        self
    }
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_owned());
        self
    }
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_owned());
        self
    }
    pub fn issue_date(mut self, issue_date: &str) -> Self {
        self.issue_date = Some(issue_date.to_owned());
        self
    }
    pub fn line_items(mut self, line_items: Vec<serde_json::Value>) -> Self {
        self.line_items = Some(line_items);
        self
    }
}
