use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateClientRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub address: Option<String>,
    pub currency: Option<String>,
}
impl<'a> CreateClientRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Client> {
        let mut r = self.client.client.post("/clients");
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.address {
            r = r.push_json(json!({ "address" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.currency {
            r = r.push_json(json!({ "currency" : unwrapped }));
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn address(mut self, address: &str) -> Self {
        self.address = Some(address.to_owned());
        self
    }
    pub fn currency(mut self, currency: &str) -> Self {
        self.currency = Some(currency.to_owned());
        self
    }
}
