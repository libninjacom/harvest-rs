use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateTimeEntryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub time_entry_id: String,
    pub project_id: Option<i64>,
    pub task_id: Option<i64>,
    pub spent_date: Option<String>,
    pub started_time: Option<String>,
    pub ended_time: Option<String>,
    pub hours: Option<f64>,
    pub notes: Option<String>,
    pub external_reference: Option<serde_json::Value>,
}
impl<'a> UpdateTimeEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeEntry> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/time_entries/{time_entry_id}", time_entry_id = self.time_entry_id
                ),
            );
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_json(json!({ "project_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.task_id {
            r = r.push_json(json!({ "task_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.spent_date {
            r = r.push_json(json!({ "spent_date" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.started_time {
            r = r.push_json(json!({ "started_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ended_time {
            r = r.push_json(json!({ "ended_time" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hours {
            r = r.push_json(json!({ "hours" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.notes {
            r = r.push_json(json!({ "notes" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.external_reference {
            r = r.push_json(json!({ "external_reference" : unwrapped }));
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
    pub fn task_id(mut self, task_id: i64) -> Self {
        self.task_id = Some(task_id);
        self
    }
    pub fn spent_date(mut self, spent_date: &str) -> Self {
        self.spent_date = Some(spent_date.to_owned());
        self
    }
    pub fn started_time(mut self, started_time: &str) -> Self {
        self.started_time = Some(started_time.to_owned());
        self
    }
    pub fn ended_time(mut self, ended_time: &str) -> Self {
        self.ended_time = Some(ended_time.to_owned());
        self
    }
    pub fn hours(mut self, hours: f64) -> Self {
        self.hours = Some(hours);
        self
    }
    pub fn notes(mut self, notes: &str) -> Self {
        self.notes = Some(notes.to_owned());
        self
    }
    pub fn external_reference(mut self, external_reference: serde_json::Value) -> Self {
        self.external_reference = Some(external_reference);
        self
    }
}
