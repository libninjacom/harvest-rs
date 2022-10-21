use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RestartStoppedTimeEntryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub time_entry_id: String,
}
impl<'a> RestartStoppedTimeEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeEntry> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/time_entries/{time_entry_id}/restart", time_entry_id = self
                    .time_entry_id
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
