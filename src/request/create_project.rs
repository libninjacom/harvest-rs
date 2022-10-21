use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: Option<i64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub is_active: Option<bool>,
    pub is_billable: Option<bool>,
    pub is_fixed_fee: Option<bool>,
    pub bill_by: Option<String>,
    pub hourly_rate: Option<f64>,
    pub budget: Option<f64>,
    pub budget_by: Option<String>,
    pub budget_is_monthly: Option<bool>,
    pub notify_when_over_budget: Option<bool>,
    pub over_budget_notification_percentage: Option<f64>,
    pub show_budget_to_all: Option<bool>,
    pub cost_budget: Option<f64>,
    pub cost_budget_include_expenses: Option<bool>,
    pub fee: Option<f64>,
    pub notes: Option<String>,
    pub starts_on: Option<String>,
    pub ends_on: Option<String>,
}
impl<'a> CreateProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Project> {
        let mut r = self.client.client.post("/projects");
        if let Some(ref unwrapped) = self.client_id {
            r = r.push_json(json!({ "client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.code {
            r = r.push_json(json!({ "code" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_billable {
            r = r.push_json(json!({ "is_billable" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_fixed_fee {
            r = r.push_json(json!({ "is_fixed_fee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.bill_by {
            r = r.push_json(json!({ "bill_by" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hourly_rate {
            r = r.push_json(json!({ "hourly_rate" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.budget {
            r = r.push_json(json!({ "budget" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.budget_by {
            r = r.push_json(json!({ "budget_by" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.budget_is_monthly {
            r = r.push_json(json!({ "budget_is_monthly" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notify_when_over_budget {
            r = r.push_json(json!({ "notify_when_over_budget" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.over_budget_notification_percentage {
            r = r
                .push_json(json!({ "over_budget_notification_percentage" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.show_budget_to_all {
            r = r.push_json(json!({ "show_budget_to_all" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cost_budget {
            r = r.push_json(json!({ "cost_budget" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cost_budget_include_expenses {
            r = r.push_json(json!({ "cost_budget_include_expenses" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fee {
            r = r.push_json(json!({ "fee" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notes {
            r = r.push_json(json!({ "notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.starts_on {
            r = r.push_json(json!({ "starts_on" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ends_on {
            r = r.push_json(json!({ "ends_on" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn code(mut self, code: &str) -> Self {
        self.code = Some(code.to_owned());
        self
    }
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn is_billable(mut self, is_billable: bool) -> Self {
        self.is_billable = Some(is_billable);
        self
    }
    pub fn is_fixed_fee(mut self, is_fixed_fee: bool) -> Self {
        self.is_fixed_fee = Some(is_fixed_fee);
        self
    }
    pub fn bill_by(mut self, bill_by: &str) -> Self {
        self.bill_by = Some(bill_by.to_owned());
        self
    }
    pub fn hourly_rate(mut self, hourly_rate: f64) -> Self {
        self.hourly_rate = Some(hourly_rate);
        self
    }
    pub fn budget(mut self, budget: f64) -> Self {
        self.budget = Some(budget);
        self
    }
    pub fn budget_by(mut self, budget_by: &str) -> Self {
        self.budget_by = Some(budget_by.to_owned());
        self
    }
    pub fn budget_is_monthly(mut self, budget_is_monthly: bool) -> Self {
        self.budget_is_monthly = Some(budget_is_monthly);
        self
    }
    pub fn notify_when_over_budget(mut self, notify_when_over_budget: bool) -> Self {
        self.notify_when_over_budget = Some(notify_when_over_budget);
        self
    }
    pub fn over_budget_notification_percentage(
        mut self,
        over_budget_notification_percentage: f64,
    ) -> Self {
        self
            .over_budget_notification_percentage = Some(
            over_budget_notification_percentage,
        );
        self
    }
    pub fn show_budget_to_all(mut self, show_budget_to_all: bool) -> Self {
        self.show_budget_to_all = Some(show_budget_to_all);
        self
    }
    pub fn cost_budget(mut self, cost_budget: f64) -> Self {
        self.cost_budget = Some(cost_budget);
        self
    }
    pub fn cost_budget_include_expenses(
        mut self,
        cost_budget_include_expenses: bool,
    ) -> Self {
        self.cost_budget_include_expenses = Some(cost_budget_include_expenses);
        self
    }
    pub fn fee(mut self, fee: f64) -> Self {
        self.fee = Some(fee);
        self
    }
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_owned());
        self
    }
    pub fn starts_on(mut self, starts_on: &str) -> Self {
        self.starts_on = Some(starts_on.to_owned());
        self
    }
    pub fn ends_on(mut self, ends_on: &str) -> Self {
        self.ends_on = Some(ends_on.to_owned());
        self
    }
}
