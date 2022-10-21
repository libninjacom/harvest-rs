use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateUserAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub user_id: Option<i64>,
    pub is_active: Option<bool>,
    pub is_project_manager: Option<bool>,
    pub use_default_rates: Option<bool>,
    pub hourly_rate: Option<f64>,
    pub budget: Option<f64>,
}
impl<'a> CreateUserAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserAssignment> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/projects/{project_id}/user_assignments", project_id = self
                    .project_id
                ),
            );
        if let Some(ref unwrapped) = self.user_id {
            r = r.push_json(json!({ "user_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_project_manager {
            r = r.push_json(json!({ "is_project_manager" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.use_default_rates {
            r = r.push_json(json!({ "use_default_rates" : unwrapped }));
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
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = Some(user_id);
        self
    }
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn is_project_manager(mut self, is_project_manager: bool) -> Self {
        self.is_project_manager = Some(is_project_manager);
        self
    }
    pub fn use_default_rates(mut self, use_default_rates: bool) -> Self {
        self.use_default_rates = Some(use_default_rates);
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
