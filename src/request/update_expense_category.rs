use serde_json::json;
use crate::model::*;
use crate::HarvestClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateExpenseCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_category_id: String,
    pub name: Option<String>,
    pub unit_name: Option<String>,
    pub unit_price: Option<f64>,
    pub is_active: Option<bool>,
}
impl<'a> UpdateExpenseCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseCategory> {
        let mut r = self
            .client
            .client
            .patch(
                &format!(
                    "/expense_categories/{expense_category_id}", expense_category_id =
                    self.expense_category_id
                ),
            );
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.unit_name {
            r = r.push_json(json!({ "unit_name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.unit_price {
            r = r.push_json(json!({ "unit_price" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.is_active {
            r = r.push_json(json!({ "is_active" : unwrapped }));
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
    pub fn unit_name(mut self, unit_name: &str) -> Self {
        self.unit_name = Some(unit_name.to_owned());
        self
    }
    pub fn unit_price(mut self, unit_price: f64) -> Self {
        self.unit_price = Some(unit_price);
        self
    }
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
}
