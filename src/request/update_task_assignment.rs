use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateTaskAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub task_assignment_id: String,
    pub is_active: Option<bool>,
    pub billable: Option<bool>,
    pub hourly_rate: Option<f64>,
    pub budget: Option<f64>,
}
impl<'a> UpdateTaskAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaskAssignment> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/projects/{project_id}/task_assignments/{task_assignment_id}",
                    project_id = self.project_id, task_assignment_id = self
                    .task_assignment_id
                ),
            );
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.billable {
            r = r.push_json(json!({ "billable" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.hourly_rate {
            r = r.push_json(json!({ "hourly_rate" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.budget {
            r = r.push_json(json!({ "budget" : unwrapped }));
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn billable(mut self, billable: bool) -> Self {
        self.billable = Some(billable);
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
}
