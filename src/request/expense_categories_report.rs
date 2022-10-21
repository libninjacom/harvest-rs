use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ExpenseCategoriesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ExpenseCategoriesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/categories");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
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
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn per_page(mut self, per_page: i64) -> Self {
        self.per_page = Some(per_page);
        self
    }
}
