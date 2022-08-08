use serde_json::json;
use crate::model;
use crate::model::*;
use crate::HarvestClient;
pub struct ListClientsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListClientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Clients> {
        let mut r = self.client.client.get("/clients");
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateClientRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
    pub is_active: bool,
    pub address: String,
    pub currency: String,
}
impl<'a> CreateClientRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Client> {
        let mut r = self.client.client.post("/clients");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
        r = r.push_json(json!({ "address" : self.address }));
        r = r.push_json(json!({ "currency" : self.currency }));
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
pub struct RetrieveCompanyRequest<'a> {
    pub(crate) client: &'a HarvestClient,
}
impl<'a> RetrieveCompanyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Company> {
        let mut r = self.client.client.get("/company");
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
pub struct ListContactsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: Option<i64>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Contacts> {
        let mut r = self.client.client.get("/contacts");
        if let Some(ref client_id) = self.client_id {
            r = r.push_query("client_id", &client_id.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateContactRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub title: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_office: String,
    pub phone_mobile: String,
    pub fax: String,
}
impl<'a> CreateContactRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Contact> {
        let mut r = self.client.client.post("/contacts");
        r = r.push_json(json!({ "client_id" : self.client_id }));
        r = r.push_json(json!({ "title" : self.title }));
        r = r.push_json(json!({ "first_name" : self.first_name }));
        r = r.push_json(json!({ "last_name" : self.last_name }));
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "phone_office" : self.phone_office }));
        r = r.push_json(json!({ "phone_mobile" : self.phone_mobile }));
        r = r.push_json(json!({ "fax" : self.fax }));
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
pub struct RetrieveContactRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub contact_id: String,
}
impl<'a> RetrieveContactRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Contact> {
        let mut r = self
            .client
            .client
            .get(&format!("/contacts/{contact_id}", contact_id = self.contact_id));
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
pub struct ListEstimateItemCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListEstimateItemCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EstimateItemCategories> {
        let mut r = self.client.client.get("/estimate_item_categories");
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateEstimateItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
}
impl<'a> CreateEstimateItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EstimateItemCategory> {
        let mut r = self.client.client.post("/estimate_item_categories");
        r = r.push_json(json!({ "name" : self.name }));
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
pub struct RetrieveEstimateItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_item_category_id: String,
}
impl<'a> RetrieveEstimateItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EstimateItemCategory> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/estimate_item_categories/{estimate_item_category_id}",
                    estimate_item_category_id = self.estimate_item_category_id
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
pub struct ListEstimatesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: Option<i64>,
    pub updated_since: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub state: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListEstimatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Estimates> {
        let mut r = self.client.client.get("/estimates");
        if let Some(ref client_id) = self.client_id {
            r = r.push_query("client_id", &client_id.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref from) = self.from {
            r = r.push_query("from", &from.to_string());
        }
        if let Some(ref to) = self.to {
            r = r.push_query("to", &to.to_string());
        }
        if let Some(ref state) = self.state {
            r = r.push_query("state", &state.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
        self
    }
    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
    pub fn to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }
    pub fn state(mut self, state: String) -> Self {
        self.state = Some(state);
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
pub struct CreateEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub number: String,
    pub purchase_order: String,
    pub tax: f64,
    pub tax2: f64,
    pub discount: f64,
    pub subject: String,
    pub notes: String,
    pub currency: String,
    pub issue_date: String,
    pub line_items: Vec<serde_json::Value>,
}
impl<'a> CreateEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Estimate> {
        let mut r = self.client.client.post("/estimates");
        r = r.push_json(json!({ "client_id" : self.client_id }));
        r = r.push_json(json!({ "number" : self.number }));
        r = r.push_json(json!({ "purchase_order" : self.purchase_order }));
        r = r.push_json(json!({ "tax" : self.tax }));
        r = r.push_json(json!({ "tax2" : self.tax2 }));
        r = r.push_json(json!({ "discount" : self.discount }));
        r = r.push_json(json!({ "subject" : self.subject }));
        r = r.push_json(json!({ "notes" : self.notes }));
        r = r.push_json(json!({ "currency" : self.currency }));
        r = r.push_json(json!({ "issue_date" : self.issue_date }));
        r = r.push_json(json!({ "line_items" : self.line_items }));
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
pub struct RetrieveEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
}
impl<'a> RetrieveEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Estimate> {
        let mut r = self
            .client
            .client
            .get(&format!("/estimates/{estimate_id}", estimate_id = self.estimate_id));
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
pub struct ListMessagesForEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListMessagesForEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EstimateMessages> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/estimates/{estimate_id}/messages", estimate_id = self.estimate_id
                ),
            );
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateEstimateMessageRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
    pub event_type: String,
    pub recipients: Vec<serde_json::Value>,
    pub subject: String,
    pub body: String,
    pub send_me_a_copy: bool,
}
impl<'a> CreateEstimateMessageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EstimateMessage> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/estimates/{estimate_id}/messages", estimate_id = self.estimate_id
                ),
            );
        r = r.push_json(json!({ "event_type" : self.event_type }));
        r = r.push_json(json!({ "recipients" : self.recipients }));
        r = r.push_json(json!({ "subject" : self.subject }));
        r = r.push_json(json!({ "body" : self.body }));
        r = r.push_json(json!({ "send_me_a_copy" : self.send_me_a_copy }));
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
pub struct ListExpenseCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListExpenseCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseCategories> {
        let mut r = self.client.client.get("/expense_categories");
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateExpenseCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
    pub unit_name: String,
    pub unit_price: f64,
    pub is_active: bool,
}
impl<'a> CreateExpenseCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseCategory> {
        let mut r = self.client.client.post("/expense_categories");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "unit_name" : self.unit_name }));
        r = r.push_json(json!({ "unit_price" : self.unit_price }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
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
pub struct RetrieveExpenseCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_category_id: String,
}
impl<'a> RetrieveExpenseCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseCategory> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/expense_categories/{expense_category_id}", expense_category_id =
                    self.expense_category_id
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
        if let Some(ref user_id) = self.user_id {
            r = r.push_query("user_id", &user_id.to_string());
        }
        if let Some(ref client_id) = self.client_id {
            r = r.push_query("client_id", &client_id.to_string());
        }
        if let Some(ref project_id) = self.project_id {
            r = r.push_query("project_id", &project_id.to_string());
        }
        if let Some(ref is_billed) = self.is_billed {
            r = r.push_query("is_billed", &is_billed.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref from) = self.from {
            r = r.push_query("from", &from.to_string());
        }
        if let Some(ref to) = self.to {
            r = r.push_query("to", &to.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
        self
    }
    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
    pub fn to(mut self, to: String) -> Self {
        self.to = Some(to);
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
pub struct CreateExpenseRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: i64,
    pub project_id: i64,
    pub expense_category_id: i64,
    pub spent_date: String,
    pub units: i64,
    pub total_cost: f64,
    pub notes: String,
    pub billable: bool,
    pub receipt: String,
}
impl<'a> CreateExpenseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Expense> {
        let mut r = self.client.client.post("/expenses");
        r = r.push_json(json!({ "user_id" : self.user_id }));
        r = r.push_json(json!({ "project_id" : self.project_id }));
        r = r.push_json(json!({ "expense_category_id" : self.expense_category_id }));
        r = r.push_json(json!({ "spent_date" : self.spent_date }));
        r = r.push_json(json!({ "units" : self.units }));
        r = r.push_json(json!({ "total_cost" : self.total_cost }));
        r = r.push_json(json!({ "notes" : self.notes }));
        r = r.push_json(json!({ "billable" : self.billable }));
        r = r.push_json(json!({ "receipt" : self.receipt }));
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
pub struct RetrieveExpenseRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_id: String,
}
impl<'a> RetrieveExpenseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Expense> {
        let mut r = self
            .client
            .client
            .get(&format!("/expenses/{expense_id}", expense_id = self.expense_id));
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
pub struct ListInvoiceItemCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListInvoiceItemCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceItemCategories> {
        let mut r = self.client.client.get("/invoice_item_categories");
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
}
impl<'a> CreateInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceItemCategory> {
        let mut r = self.client.client.post("/invoice_item_categories");
        r = r.push_json(json!({ "name" : self.name }));
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
pub struct RetrieveInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_item_category_id: String,
}
impl<'a> RetrieveInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceItemCategory> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/invoice_item_categories/{invoice_item_category_id}",
                    invoice_item_category_id = self.invoice_item_category_id
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
pub struct ListInvoicesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: Option<i64>,
    pub project_id: Option<i64>,
    pub updated_since: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub state: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListInvoicesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoices> {
        let mut r = self.client.client.get("/invoices");
        if let Some(ref client_id) = self.client_id {
            r = r.push_query("client_id", &client_id.to_string());
        }
        if let Some(ref project_id) = self.project_id {
            r = r.push_query("project_id", &project_id.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref from) = self.from {
            r = r.push_query("from", &from.to_string());
        }
        if let Some(ref to) = self.to {
            r = r.push_query("to", &to.to_string());
        }
        if let Some(ref state) = self.state {
            r = r.push_query("state", &state.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn project_id(mut self, project_id: i64) -> Self {
        self.project_id = Some(project_id);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
        self
    }
    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
    pub fn to(mut self, to: String) -> Self {
        self.to = Some(to);
        self
    }
    pub fn state(mut self, state: String) -> Self {
        self.state = Some(state);
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
pub struct CreateInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub retainer_id: i64,
    pub estimate_id: i64,
    pub number: String,
    pub purchase_order: String,
    pub tax: f64,
    pub tax2: f64,
    pub discount: f64,
    pub subject: String,
    pub notes: String,
    pub currency: String,
    pub issue_date: String,
    pub due_date: String,
    pub payment_term: String,
    pub line_items_import: Option<serde_json::Value>,
    pub line_items: Vec<serde_json::Value>,
}
impl<'a> CreateInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self.client.client.post("/invoices");
        r = r.push_json(json!({ "client_id" : self.client_id }));
        r = r.push_json(json!({ "retainer_id" : self.retainer_id }));
        r = r.push_json(json!({ "estimate_id" : self.estimate_id }));
        r = r.push_json(json!({ "number" : self.number }));
        r = r.push_json(json!({ "purchase_order" : self.purchase_order }));
        r = r.push_json(json!({ "tax" : self.tax }));
        r = r.push_json(json!({ "tax2" : self.tax2 }));
        r = r.push_json(json!({ "discount" : self.discount }));
        r = r.push_json(json!({ "subject" : self.subject }));
        r = r.push_json(json!({ "notes" : self.notes }));
        r = r.push_json(json!({ "currency" : self.currency }));
        r = r.push_json(json!({ "issue_date" : self.issue_date }));
        r = r.push_json(json!({ "due_date" : self.due_date }));
        r = r.push_json(json!({ "payment_term" : self.payment_term }));
        if let Some(ref line_items_import) = self.line_items_import {
            r = r.push_json(json!({ "line_items_import" : line_items_import }));
        }
        r = r.push_json(json!({ "line_items" : self.line_items }));
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
    pub fn line_items_import(mut self, line_items_import: serde_json::Value) -> Self {
        self.line_items_import = Some(line_items_import);
        self
    }
}
pub struct RetrieveInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
}
impl<'a> RetrieveInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Invoice> {
        let mut r = self
            .client
            .client
            .get(&format!("/invoices/{invoice_id}", invoice_id = self.invoice_id));
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
pub struct ListMessagesForInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListMessagesForInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceMessages> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/invoices/{invoice_id}/messages", invoice_id = self.invoice_id),
            );
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateInvoiceMessageRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub event_type: String,
    pub recipients: Vec<serde_json::Value>,
    pub subject: String,
    pub body: String,
    pub include_link_to_client_invoice: bool,
    pub attach_pdf: bool,
    pub send_me_a_copy: bool,
    pub thank_you: bool,
}
impl<'a> CreateInvoiceMessageRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoiceMessage> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/messages", invoice_id = self.invoice_id),
            );
        r = r.push_json(json!({ "event_type" : self.event_type }));
        r = r.push_json(json!({ "recipients" : self.recipients }));
        r = r.push_json(json!({ "subject" : self.subject }));
        r = r.push_json(json!({ "body" : self.body }));
        r = r
            .push_json(
                json!(
                    { "include_link_to_client_invoice" : self
                    .include_link_to_client_invoice }
                ),
            );
        r = r.push_json(json!({ "attach_pdf" : self.attach_pdf }));
        r = r.push_json(json!({ "send_me_a_copy" : self.send_me_a_copy }));
        r = r.push_json(json!({ "thank_you" : self.thank_you }));
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
pub struct ListPaymentsForInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListPaymentsForInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoicePayments> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/invoices/{invoice_id}/payments", invoice_id = self.invoice_id),
            );
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateInvoicePaymentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub amount: f64,
    pub paid_at: String,
    pub paid_date: String,
    pub notes: String,
}
impl<'a> CreateInvoicePaymentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<InvoicePayment> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/payments", invoice_id = self.invoice_id),
            );
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "paid_at" : self.paid_at }));
        r = r.push_json(json!({ "paid_date" : self.paid_date }));
        r = r.push_json(json!({ "notes" : self.notes }));
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
pub struct ListProjectsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: Option<bool>,
    pub client_id: Option<i64>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListProjectsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Projects> {
        let mut r = self.client.client.get("/projects");
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref client_id) = self.client_id {
            r = r.push_query("client_id", &client_id.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn client_id(mut self, client_id: i64) -> Self {
        self.client_id = Some(client_id);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub name: String,
    pub code: String,
    pub is_active: bool,
    pub is_billable: bool,
    pub is_fixed_fee: bool,
    pub bill_by: String,
    pub hourly_rate: f64,
    pub budget: f64,
    pub budget_by: String,
    pub budget_is_monthly: bool,
    pub notify_when_over_budget: bool,
    pub over_budget_notification_percentage: f64,
    pub show_budget_to_all: bool,
    pub cost_budget: f64,
    pub cost_budget_include_expenses: bool,
    pub fee: f64,
    pub notes: String,
    pub starts_on: String,
    pub ends_on: String,
}
impl<'a> CreateProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Project> {
        let mut r = self.client.client.post("/projects");
        r = r.push_json(json!({ "client_id" : self.client_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "code" : self.code }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
        r = r.push_json(json!({ "is_billable" : self.is_billable }));
        r = r.push_json(json!({ "is_fixed_fee" : self.is_fixed_fee }));
        r = r.push_json(json!({ "bill_by" : self.bill_by }));
        r = r.push_json(json!({ "hourly_rate" : self.hourly_rate }));
        r = r.push_json(json!({ "budget" : self.budget }));
        r = r.push_json(json!({ "budget_by" : self.budget_by }));
        r = r.push_json(json!({ "budget_is_monthly" : self.budget_is_monthly }));
        r = r
            .push_json(
                json!({ "notify_when_over_budget" : self.notify_when_over_budget }),
            );
        r = r
            .push_json(
                json!(
                    { "over_budget_notification_percentage" : self
                    .over_budget_notification_percentage }
                ),
            );
        r = r.push_json(json!({ "show_budget_to_all" : self.show_budget_to_all }));
        r = r.push_json(json!({ "cost_budget" : self.cost_budget }));
        r = r
            .push_json(
                json!(
                    { "cost_budget_include_expenses" : self.cost_budget_include_expenses
                    }
                ),
            );
        r = r.push_json(json!({ "fee" : self.fee }));
        r = r.push_json(json!({ "notes" : self.notes }));
        r = r.push_json(json!({ "starts_on" : self.starts_on }));
        r = r.push_json(json!({ "ends_on" : self.ends_on }));
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
pub struct RetrieveProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
}
impl<'a> RetrieveProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Project> {
        let mut r = self
            .client
            .client
            .get(&format!("/projects/{project_id}", project_id = self.project_id));
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
pub struct ListTaskAssignmentsForSpecificProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListTaskAssignmentsForSpecificProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaskAssignments> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/projects/{project_id}/task_assignments", project_id = self
                    .project_id
                ),
            );
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateTaskAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub task_id: i64,
    pub is_active: bool,
    pub billable: bool,
    pub hourly_rate: f64,
    pub budget: f64,
}
impl<'a> CreateTaskAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaskAssignment> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/projects/{project_id}/task_assignments", project_id = self
                    .project_id
                ),
            );
        r = r.push_json(json!({ "task_id" : self.task_id }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
        r = r.push_json(json!({ "billable" : self.billable }));
        r = r.push_json(json!({ "hourly_rate" : self.hourly_rate }));
        r = r.push_json(json!({ "budget" : self.budget }));
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
pub struct RetrieveTaskAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub task_assignment_id: String,
}
impl<'a> RetrieveTaskAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaskAssignment> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/projects/{project_id}/task_assignments/{task_assignment_id}",
                    project_id = self.project_id, task_assignment_id = self
                    .task_assignment_id
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
pub struct ListUserAssignmentsForSpecificProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub user_id: Option<i64>,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListUserAssignmentsForSpecificProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserAssignments> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/projects/{project_id}/user_assignments", project_id = self
                    .project_id
                ),
            );
        if let Some(ref user_id) = self.user_id {
            r = r.push_query("user_id", &user_id.to_string());
        }
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateUserAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub user_id: i64,
    pub is_active: bool,
    pub is_project_manager: bool,
    pub use_default_rates: bool,
    pub hourly_rate: f64,
    pub budget: f64,
}
impl<'a> CreateUserAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserAssignment> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/projects/{project_id}/user_assignments", project_id = self
                    .project_id
                ),
            );
        r = r.push_json(json!({ "user_id" : self.user_id }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
        r = r.push_json(json!({ "is_project_manager" : self.is_project_manager }));
        r = r.push_json(json!({ "use_default_rates" : self.use_default_rates }));
        r = r.push_json(json!({ "hourly_rate" : self.hourly_rate }));
        r = r.push_json(json!({ "budget" : self.budget }));
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
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct ClientsExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ClientsExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/clients");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct ProjectsExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ProjectsExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/projects");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct TeamExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> TeamExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/team");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct ProjectBudgetReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub is_active: Option<bool>,
}
impl<'a> ProjectBudgetReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProjectBudgetReportResults> {
        let mut r = self.client.client.get("/reports/project_budget");
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
        }
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
}
pub struct ClientsTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ClientsTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/clients");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct ProjectsTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ProjectsTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/projects");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct TasksReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> TasksReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/tasks");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct TeamTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> TeamTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/team");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct UninvoicedReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> UninvoicedReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UninvoicedReportResults> {
        let mut r = self.client.client.get("/reports/uninvoiced");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct ListRolesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListRolesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Roles> {
        let mut r = self.client.client.get("/roles");
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct CreateRoleRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
    pub user_ids: Vec<i64>,
}
impl<'a> CreateRoleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Role> {
        let mut r = self.client.client.post("/roles");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "user_ids" : self.user_ids }));
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
pub struct RetrieveRoleRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub role_id: String,
}
impl<'a> RetrieveRoleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Role> {
        let mut r = self
            .client
            .client
            .get(&format!("/roles/{role_id}", role_id = self.role_id));
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
pub struct ListTaskAssignmentsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListTaskAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TaskAssignments> {
        let mut r = self.client.client.get("/task_assignments");
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct ListTasksRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListTasksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Tasks> {
        let mut r = self.client.client.get("/tasks");
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateTaskRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
    pub billable_by_default: bool,
    pub default_hourly_rate: f64,
    pub is_default: bool,
    pub is_active: bool,
}
impl<'a> CreateTaskRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Task> {
        let mut r = self.client.client.post("/tasks");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "billable_by_default" : self.billable_by_default }));
        r = r.push_json(json!({ "default_hourly_rate" : self.default_hourly_rate }));
        r = r.push_json(json!({ "is_default" : self.is_default }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
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
pub struct RetrieveTaskRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub task_id: String,
}
impl<'a> RetrieveTaskRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Task> {
        let mut r = self
            .client
            .client
            .get(&format!("/tasks/{task_id}", task_id = self.task_id));
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
pub struct ListTimeEntriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: Option<i64>,
    pub client_id: Option<i64>,
    pub project_id: Option<i64>,
    pub task_id: Option<i64>,
    pub external_reference_id: Option<String>,
    pub is_billed: Option<bool>,
    pub is_running: Option<bool>,
    pub updated_since: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListTimeEntriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeEntries> {
        let mut r = self.client.client.get("/time_entries");
        if let Some(ref user_id) = self.user_id {
            r = r.push_query("user_id", &user_id.to_string());
        }
        if let Some(ref client_id) = self.client_id {
            r = r.push_query("client_id", &client_id.to_string());
        }
        if let Some(ref project_id) = self.project_id {
            r = r.push_query("project_id", &project_id.to_string());
        }
        if let Some(ref task_id) = self.task_id {
            r = r.push_query("task_id", &task_id.to_string());
        }
        if let Some(ref external_reference_id) = self.external_reference_id {
            r = r
                .push_query("external_reference_id", &external_reference_id.to_string());
        }
        if let Some(ref is_billed) = self.is_billed {
            r = r.push_query("is_billed", &is_billed.to_string());
        }
        if let Some(ref is_running) = self.is_running {
            r = r.push_query("is_running", &is_running.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref from) = self.from {
            r = r.push_query("from", &from.to_string());
        }
        if let Some(ref to) = self.to {
            r = r.push_query("to", &to.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn task_id(mut self, task_id: i64) -> Self {
        self.task_id = Some(task_id);
        self
    }
    pub fn external_reference_id(mut self, external_reference_id: String) -> Self {
        self.external_reference_id = Some(external_reference_id);
        self
    }
    pub fn is_billed(mut self, is_billed: bool) -> Self {
        self.is_billed = Some(is_billed);
        self
    }
    pub fn is_running(mut self, is_running: bool) -> Self {
        self.is_running = Some(is_running);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
        self
    }
    pub fn from(mut self, from: String) -> Self {
        self.from = Some(from);
        self
    }
    pub fn to(mut self, to: String) -> Self {
        self.to = Some(to);
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
pub struct CreateTimeEntryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: i64,
    pub project_id: i64,
    pub task_id: i64,
    pub spent_date: String,
    pub started_time: String,
    pub ended_time: String,
    pub notes: String,
    pub external_reference: Option<serde_json::Value>,
    pub hours: f64,
}
impl<'a> CreateTimeEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeEntry> {
        let mut r = self.client.client.post("/time_entries");
        r = r.push_json(json!({ "user_id" : self.user_id }));
        r = r.push_json(json!({ "project_id" : self.project_id }));
        r = r.push_json(json!({ "task_id" : self.task_id }));
        r = r.push_json(json!({ "spent_date" : self.spent_date }));
        r = r.push_json(json!({ "started_time" : self.started_time }));
        r = r.push_json(json!({ "ended_time" : self.ended_time }));
        r = r.push_json(json!({ "notes" : self.notes }));
        if let Some(ref external_reference) = self.external_reference {
            r = r.push_json(json!({ "external_reference" : external_reference }));
        }
        r = r.push_json(json!({ "hours" : self.hours }));
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
    pub fn external_reference(mut self, external_reference: serde_json::Value) -> Self {
        self.external_reference = Some(external_reference);
        self
    }
}
pub struct RetrieveTimeEntryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub time_entry_id: String,
}
impl<'a> RetrieveTimeEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<TimeEntry> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/time_entries/{time_entry_id}", time_entry_id = self.time_entry_id
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
pub struct ListUserAssignmentsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: Option<i64>,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListUserAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UserAssignments> {
        let mut r = self.client.client.get("/user_assignments");
        if let Some(ref user_id) = self.user_id {
            r = r.push_query("user_id", &user_id.to_string());
        }
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct ListUsersRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: Option<bool>,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListUsersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Users> {
        let mut r = self.client.client.get("/users");
        if let Some(ref is_active) = self.is_active {
            r = r.push_query("is_active", &is_active.to_string());
        }
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
pub struct CreateUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timezone: String,
    pub has_access_to_all_future_projects: bool,
    pub is_contractor: bool,
    pub is_active: bool,
    pub weekly_capacity: i64,
    pub default_hourly_rate: f64,
    pub cost_rate: f64,
    pub roles: Vec<String>,
}
impl<'a> CreateUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<User> {
        let mut r = self.client.client.post("/users");
        r = r.push_json(json!({ "first_name" : self.first_name }));
        r = r.push_json(json!({ "last_name" : self.last_name }));
        r = r.push_json(json!({ "email" : self.email }));
        r = r.push_json(json!({ "timezone" : self.timezone }));
        r = r
            .push_json(
                json!(
                    { "has_access_to_all_future_projects" : self
                    .has_access_to_all_future_projects }
                ),
            );
        r = r.push_json(json!({ "is_contractor" : self.is_contractor }));
        r = r.push_json(json!({ "is_active" : self.is_active }));
        r = r.push_json(json!({ "weekly_capacity" : self.weekly_capacity }));
        r = r.push_json(json!({ "default_hourly_rate" : self.default_hourly_rate }));
        r = r.push_json(json!({ "cost_rate" : self.cost_rate }));
        r = r.push_json(json!({ "roles" : self.roles }));
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
pub struct RetrieveTheCurrentlyAuthenticatedUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
}
impl<'a> RetrieveTheCurrentlyAuthenticatedUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<User> {
        let mut r = self.client.client.get("/users/me");
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
pub struct ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProjectAssignments> {
        let mut r = self.client.client.get("/users/me/project_assignments");
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct RetrieveUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
}
impl<'a> RetrieveUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<User> {
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}", user_id = self.user_id));
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
pub struct ListBillableRatesForSpecificUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListBillableRatesForSpecificUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillableRates> {
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/billable_rates", user_id = self.user_id));
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct CreateBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: f64,
    pub start_date: String,
}
impl<'a> CreateBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillableRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/users/{user_id}/billable_rates", user_id = self.user_id));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "start_date" : self.start_date }));
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
pub struct RetrieveBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub billable_rate_id: String,
}
impl<'a> RetrieveBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BillableRate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/users/{user_id}/billable_rates/{billable_rate_id}", user_id = self
                    .user_id, billable_rate_id = self.billable_rate_id
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
pub struct ListCostRatesForSpecificUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListCostRatesForSpecificUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CostRates> {
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/cost_rates", user_id = self.user_id));
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
pub struct CreateCostRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: f64,
    pub start_date: String,
}
impl<'a> CreateCostRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CostRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/users/{user_id}/cost_rates", user_id = self.user_id));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "start_date" : self.start_date }));
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
pub struct RetrieveCostRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub cost_rate_id: String,
}
impl<'a> RetrieveCostRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CostRate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/users/{user_id}/cost_rates/{cost_rate_id}", user_id = self.user_id,
                    cost_rate_id = self.cost_rate_id
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
pub struct ListActiveProjectAssignmentsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub updated_since: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}
impl<'a> ListActiveProjectAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProjectAssignments> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/users/{user_id}/project_assignments", user_id = self.user_id),
            );
        if let Some(ref updated_since) = self.updated_since {
            r = r.push_query("updated_since", &updated_since.to_string());
        }
        if let Some(ref page) = self.page {
            r = r.push_query("page", &page.to_string());
        }
        if let Some(ref per_page) = self.per_page {
            r = r.push_query("per_page", &per_page.to_string());
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
    pub fn updated_since(mut self, updated_since: String) -> Self {
        self.updated_since = Some(updated_since);
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
