use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateUserAssignedTeammatesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub teammate_ids: Option<serde_json::Value>,
}
impl<'a> UpdateUserAssignedTeammatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TeammatesPatchResponse> {
        let mut r = self
            .client
            .client
            .patch(&format!("/users/{user_id}/teammates", user_id = self.user_id));
        if let Some(ref unwrapped) = self.teammate_ids {
            r = r.push_json(json!({ "teammate_ids" : unwrapped }));
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
    pub fn teammate_ids(mut self, teammate_ids: serde_json::Value) -> Self {
        self.teammate_ids = Some(teammate_ids);
        self
    }
}
