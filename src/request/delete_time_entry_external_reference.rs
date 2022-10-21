use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteTimeEntryExternalReferenceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub time_entry_id: String,
}
impl<'a> DeleteTimeEntryExternalReferenceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<()> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/time_entries/{time_entry_id}/external_reference", time_entry_id =
                    self.time_entry_id
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => Ok(()),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
