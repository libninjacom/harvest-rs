use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RetrieveClientRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: String,
}
impl<'a> RetrieveClientRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Client> {
        let mut r = self
            .client
            .client
            .get(&format!("/clients/{client_id}", client_id = self.client_id));
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
