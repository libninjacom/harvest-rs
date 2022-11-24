use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub timezone: Option<String>,
    pub has_access_to_all_future_projects: Option<bool>,
    pub is_contractor: Option<bool>,
    pub is_active: Option<bool>,
    pub weekly_capacity: Option<i64>,
    pub default_hourly_rate: Option<f64>,
    pub cost_rate: Option<f64>,
    pub roles: Option<Vec<String>>,
    pub access_roles: Option<Vec<String>>,
}
impl<'a> CreateUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<User> {
        let mut r = self.client.client.post("/users");
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.timezone {
            r = r.push_json(json!({ "timezone" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.has_access_to_all_future_projects {
            r = r.push_json(json!({ "has_access_to_all_future_projects" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_contractor {
            r = r.push_json(json!({ "is_contractor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.weekly_capacity {
            r = r.push_json(json!({ "weekly_capacity" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.default_hourly_rate {
            r = r.push_json(json!({ "default_hourly_rate" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.cost_rate {
            r = r.push_json(json!({ "cost_rate" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.roles {
            r = r.push_json(json!({ "roles" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.access_roles {
            r = r.push_json(json!({ "access_roles" : unwrapped }));
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
    pub fn first_name(mut self, first_name: &str) -> Self {
        self.first_name = Some(first_name.to_owned());
        self
    }
    pub fn last_name(mut self, last_name: &str) -> Self {
        self.last_name = Some(last_name.to_owned());
        self
    }
    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_owned());
        self
    }
    pub fn timezone(mut self, timezone: &str) -> Self {
        self.timezone = Some(timezone.to_owned());
        self
    }
    pub fn has_access_to_all_future_projects(
        mut self,
        has_access_to_all_future_projects: bool,
    ) -> Self {
        self.has_access_to_all_future_projects = Some(has_access_to_all_future_projects);
        self
    }
    pub fn is_contractor(mut self, is_contractor: bool) -> Self {
        self.is_contractor = Some(is_contractor);
        self
    }
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn weekly_capacity(mut self, weekly_capacity: i64) -> Self {
        self.weekly_capacity = Some(weekly_capacity);
        self
    }
    pub fn default_hourly_rate(mut self, default_hourly_rate: f64) -> Self {
        self.default_hourly_rate = Some(default_hourly_rate);
        self
    }
    pub fn cost_rate(mut self, cost_rate: f64) -> Self {
        self.cost_rate = Some(cost_rate);
        self
    }
    pub fn roles(mut self, roles: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.roles = Some(roles.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn access_roles(
        mut self,
        access_roles: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .access_roles = Some(
            access_roles.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
