use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateExpenseRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_id: String,
    pub project_id: Option<i64>,
    pub expense_category_id: Option<i64>,
    pub spent_date: Option<String>,
    pub units: Option<i64>,
    pub total_cost: Option<f64>,
    pub notes: Option<String>,
    pub billable: Option<bool>,
    pub receipt: Option<String>,
    pub delete_receipt: Option<bool>,
}
impl<'a> UpdateExpenseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Expense> {
        let mut r = self
            .client
            .client
            .patch(&format!("/expenses/{expense_id}", expense_id = self.expense_id));
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_json(json!({ "project_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.expense_category_id {
            r = r.push_json(json!({ "expense_category_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.spent_date {
            r = r.push_json(json!({ "spent_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.units {
            r = r.push_json(json!({ "units" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.total_cost {
            r = r.push_json(json!({ "total_cost" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notes {
            r = r.push_json(json!({ "notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billable {
            r = r.push_json(json!({ "billable" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.receipt {
            r = r.push_json(json!({ "receipt" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.delete_receipt {
            r = r.push_json(json!({ "delete_receipt" : unwrapped }));
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
    pub fn project_id(mut self, project_id: i64) -> Self {
        self.project_id = Some(project_id);
        self
    }
    pub fn expense_category_id(mut self, expense_category_id: i64) -> Self {
        self.expense_category_id = Some(expense_category_id);
        self
    }
    pub fn spent_date(mut self, spent_date: &str) -> Self {
        self.spent_date = Some(spent_date.to_owned());
        self
    }
    pub fn units(mut self, units: i64) -> Self {
        self.units = Some(units);
        self
    }
    pub fn total_cost(mut self, total_cost: f64) -> Self {
        self.total_cost = Some(total_cost);
        self
    }
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_owned());
        self
    }
    pub fn billable(mut self, billable: bool) -> Self {
        self.billable = Some(billable);
        self
    }
    pub fn receipt(mut self, receipt: &str) -> Self {
        self.receipt = Some(receipt.to_owned());
        self
    }
    pub fn delete_receipt(mut self, delete_receipt: bool) -> Self {
        self.delete_receipt = Some(delete_receipt);
        self
    }
}
