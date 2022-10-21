use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateContactRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub contact_id: String,
    pub client_id: Option<i64>,
    pub title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_office: Option<String>,
    pub phone_mobile: Option<String>,
    pub fax: Option<String>,
}
impl<'a> UpdateContactRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Contact> {
        let mut r = self
            .client
            .client
            .patch(&format!("/contacts/{contact_id}", contact_id = self.contact_id));
        if let Some(ref unwrapped) = self.client_id {
            r = r.push_json(json!({ "client_id" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.title {
            r = r.push_json(json!({ "title" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.first_name {
            r = r.push_json(json!({ "first_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.last_name {
            r = r.push_json(json!({ "last_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.email {
            r = r.push_json(json!({ "email" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.phone_office {
            r = r.push_json(json!({ "phone_office" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.phone_mobile {
            r = r.push_json(json!({ "phone_mobile" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fax {
            r = r.push_json(json!({ "fax" : unwrapped }));
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
    pub fn client_id(mut self, client_id: i64) -> Self {
        self.client_id = Some(client_id);
        self
    }
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_owned());
        self
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
    pub fn phone_office(mut self, phone_office: &str) -> Self {
        self.phone_office = Some(phone_office.to_owned());
        self
    }
    pub fn phone_mobile(mut self, phone_mobile: &str) -> Self {
        self.phone_mobile = Some(phone_mobile.to_owned());
        self
    }
    pub fn fax(mut self, fax: &str) -> Self {
        self.fax = Some(fax.to_owned());
        self
    }
}
