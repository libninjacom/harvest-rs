use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveUserAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub user_assignment_id: String,
}
impl<'a> RetrieveUserAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserAssignment> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/projects/{project_id}/user_assignments/{user_assignment_id}",
                    project_id = self.project_id, user_assignment_id = self
                    .user_assignment_id
                ),
            );
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
}
