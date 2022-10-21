use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateRoleRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: Option<String>,
    pub user_ids: Option<Vec<i64>>,
}
impl<'a> CreateRoleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Role> {
        let mut r = self.client.client.post("/roles");
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_ids {
            r = r.push_json(json!({ "user_ids" : unwrapped }));
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn user_ids(mut self, user_ids: Vec<i64>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }
}
