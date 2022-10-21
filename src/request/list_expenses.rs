use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListExpensesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: Option<i64>,
    pub client_id: Option<i64>,
    pub project_id: Option<i64>,
    pub is_billed: Option<bool>,
    pub updated_since: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListExpensesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Expenses> {
        let mut r = self.client.client.get("/expenses");
        if let Some(ref unwrapped) = self.user_id {
            r = r.push_query("user_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.client_id {
            r = r.push_query("client_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_query("project_id", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.is_billed {
            r = r.push_query("is_billed", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.updated_since {
            r = r.push_query("updated_since", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.from {
            r = r.push_query("from", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.to {
            r = r.push_query("to", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.push_query("page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.per_page {
            r = r.push_query("per_page", &unwrapped.to_string());
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
    pub fn client_id(mut self, client_id: i64) -> Self {
        self.client_id = Some(client_id);
        self
    }
    pub fn project_id(mut self, project_id: i64) -> Self {
        self.project_id = Some(project_id);
        self
    }
    pub fn is_billed(mut self, is_billed: bool) -> Self {
        self.is_billed = Some(is_billed);
        self
    }
    pub fn updated_since(mut self, updated_since: &str) -> Self {
        self.updated_since = Some(updated_since.to_owned());
        self
    }
    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_owned());
        self
    }
    pub fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_owned());
        self
    }
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = Some(per_page);
        self
    }
}
