use serde_json::json;
use crate::model;
use crate::model::*;
use crate::HarvestClient;
pub struct ListClientsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListClientsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Clients> {
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/clients"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::Client> {
        let mut r = self.client.client.post("/clients");
        r = r
            .json(
                json!(
                    { "name" : self.name, "is_active" : self.is_active, "address" : self
                    .address, "currency" : self.currency }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Client> {
        let client_id = self.client_id;
        let mut r = self.client.client.get(&format!("/clients/{client_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct RetrieveCompanyRequest<'a> {
    pub(crate) client: &'a HarvestClient,
}
impl<'a> RetrieveCompanyRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Company> {
        let mut r = self.client.client.get("/company");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListContactsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Contacts> {
        let client_id = self.client_id;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/contacts"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::Contact> {
        let mut r = self.client.client.post("/contacts");
        r = r
            .json(
                json!(
                    { "client_id" : self.client_id, "title" : self.title, "first_name" :
                    self.first_name, "last_name" : self.last_name, "email" : self.email,
                    "phone_office" : self.phone_office, "phone_mobile" : self
                    .phone_mobile, "fax" : self.fax }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Contact> {
        let contact_id = self.contact_id;
        let mut r = self.client.client.get(&format!("/contacts/{contact_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListEstimateItemCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListEstimateItemCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateItemCategories> {
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/estimate_item_categories"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateEstimateItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
}
impl<'a> CreateEstimateItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateItemCategory> {
        let mut r = self.client.client.post("/estimate_item_categories");
        r = r.json(json!({ "name" : self.name }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::EstimateItemCategory> {
        let estimate_item_category_id = self.estimate_item_category_id;
        let mut r = self
            .client
            .client
            .get(&format!("/estimate_item_categories/{estimate_item_category_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListEstimatesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub updated_since: String,
    pub from: String,
    pub to: String,
    pub state: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListEstimatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Estimates> {
        let client_id = self.client_id;
        let updated_since = self.updated_since;
        let from = self.from;
        let to = self.to;
        let state = self.state;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/estimates"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub number: String,
    pub purchase_order: String,
    pub tax: f64,
    pub tax_2: f64,
    pub discount: f64,
    pub subject: String,
    pub notes: String,
    pub currency: String,
    pub issue_date: String,
    pub line_items: Vec<serde_json::Value>,
}
impl<'a> CreateEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Estimate> {
        let mut r = self.client.client.post("/estimates");
        r = r
            .json(
                json!(
                    { "client_id" : self.client_id, "number" : self.number,
                    "purchase_order" : self.purchase_order, "tax" : self.tax, "tax2" :
                    self.tax_2, "discount" : self.discount, "subject" : self.subject,
                    "notes" : self.notes, "currency" : self.currency, "issue_date" : self
                    .issue_date, "line_items" : self.line_items }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Estimate> {
        let estimate_id = self.estimate_id;
        let mut r = self.client.client.get(&format!("/estimates/{estimate_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListMessagesForEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListMessagesForEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateMessages> {
        let estimate_id = self.estimate_id;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self
            .client
            .client
            .get(&format!("/estimates/{estimate_id}/messages"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::EstimateMessage> {
        let estimate_id = self.estimate_id;
        let mut r = self
            .client
            .client
            .post(&format!("/estimates/{estimate_id}/messages"));
        r = r
            .json(
                json!(
                    { "event_type" : self.event_type, "recipients" : self.recipients,
                    "subject" : self.subject, "body" : self.body, "send_me_a_copy" : self
                    .send_me_a_copy }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListExpenseCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListExpenseCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseCategories> {
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/expense_categories"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::ExpenseCategory> {
        let mut r = self.client.client.post("/expense_categories");
        r = r
            .json(
                json!(
                    { "name" : self.name, "unit_name" : self.unit_name, "unit_price" :
                    self.unit_price, "is_active" : self.is_active }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::ExpenseCategory> {
        let expense_category_id = self.expense_category_id;
        let mut r = self
            .client
            .client
            .get(&format!("/expense_categories/{expense_category_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListExpensesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: i64,
    pub client_id: i64,
    pub project_id: i64,
    pub is_billed: bool,
    pub updated_since: String,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListExpensesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Expenses> {
        let user_id = self.user_id;
        let client_id = self.client_id;
        let project_id = self.project_id;
        let is_billed = self.is_billed;
        let updated_since = self.updated_since;
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/expenses"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::Expense> {
        let mut r = self.client.client.post("/expenses");
        r = r
            .json(
                json!(
                    { "user_id" : self.user_id, "project_id" : self.project_id,
                    "expense_category_id" : self.expense_category_id, "spent_date" : self
                    .spent_date, "units" : self.units, "total_cost" : self.total_cost,
                    "notes" : self.notes, "billable" : self.billable, "receipt" : self
                    .receipt }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Expense> {
        let expense_id = self.expense_id;
        let mut r = self.client.client.get(&format!("/expenses/{expense_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListInvoiceItemCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListInvoiceItemCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceItemCategories> {
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/invoice_item_categories"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
}
impl<'a> CreateInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceItemCategory> {
        let mut r = self.client.client.post("/invoice_item_categories");
        r = r.json(json!({ "name" : self.name }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::InvoiceItemCategory> {
        let invoice_item_category_id = self.invoice_item_category_id;
        let mut r = self
            .client
            .client
            .get(&format!("/invoice_item_categories/{invoice_item_category_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListInvoicesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: i64,
    pub project_id: i64,
    pub updated_since: String,
    pub from: String,
    pub to: String,
    pub state: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListInvoicesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Invoices> {
        let client_id = self.client_id;
        let project_id = self.project_id;
        let updated_since = self.updated_since;
        let from = self.from;
        let to = self.to;
        let state = self.state;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/invoices"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub tax_2: f64,
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
    pub async fn send(self) -> anyhow::Result<model::Invoice> {
        let mut r = self.client.client.post("/invoices");
        if let Some(ref line_items_import) = self.line_items_import {
            r = r.push_query("line_items_import", &line_items_import.to_string());
        }
        r = r
            .json(
                json!(
                    { "client_id" : self.client_id, "retainer_id" : self.retainer_id,
                    "estimate_id" : self.estimate_id, "number" : self.number,
                    "purchase_order" : self.purchase_order, "tax" : self.tax, "tax2" :
                    self.tax_2, "discount" : self.discount, "subject" : self.subject,
                    "notes" : self.notes, "currency" : self.currency, "issue_date" : self
                    .issue_date, "due_date" : self.due_date, "payment_term" : self
                    .payment_term, "line_items_import" : self.line_items_import,
                    "line_items" : self.line_items }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Invoice> {
        let invoice_id = self.invoice_id;
        let mut r = self.client.client.get(&format!("/invoices/{invoice_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListMessagesForInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListMessagesForInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceMessages> {
        let invoice_id = self.invoice_id;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/invoices/{invoice_id}/messages"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::InvoiceMessage> {
        let invoice_id = self.invoice_id;
        let mut r = self.client.client.post(&format!("/invoices/{invoice_id}/messages"));
        r = r
            .json(
                json!(
                    { "event_type" : self.event_type, "recipients" : self.recipients,
                    "subject" : self.subject, "body" : self.body,
                    "include_link_to_client_invoice" : self
                    .include_link_to_client_invoice, "attach_pdf" : self.attach_pdf,
                    "send_me_a_copy" : self.send_me_a_copy, "thank_you" : self.thank_you
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListPaymentsForInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListPaymentsForInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoicePayments> {
        let invoice_id = self.invoice_id;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/invoices/{invoice_id}/payments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::InvoicePayment> {
        let invoice_id = self.invoice_id;
        let mut r = self.client.client.post(&format!("/invoices/{invoice_id}/payments"));
        r = r
            .json(
                json!(
                    { "amount" : self.amount, "paid_at" : self.paid_at, "paid_date" :
                    self.paid_date, "notes" : self.notes }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListProjectsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub client_id: i64,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListProjectsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Projects> {
        let is_active = self.is_active;
        let client_id = self.client_id;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/projects"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::Project> {
        let mut r = self.client.client.post("/projects");
        r = r
            .json(
                json!(
                    { "client_id" : self.client_id, "name" : self.name, "code" : self
                    .code, "is_active" : self.is_active, "is_billable" : self
                    .is_billable, "is_fixed_fee" : self.is_fixed_fee, "bill_by" : self
                    .bill_by, "hourly_rate" : self.hourly_rate, "budget" : self.budget,
                    "budget_by" : self.budget_by, "budget_is_monthly" : self
                    .budget_is_monthly, "notify_when_over_budget" : self
                    .notify_when_over_budget, "over_budget_notification_percentage" :
                    self.over_budget_notification_percentage, "show_budget_to_all" : self
                    .show_budget_to_all, "cost_budget" : self.cost_budget,
                    "cost_budget_include_expenses" : self.cost_budget_include_expenses,
                    "fee" : self.fee, "notes" : self.notes, "starts_on" : self.starts_on,
                    "ends_on" : self.ends_on }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Project> {
        let project_id = self.project_id;
        let mut r = self.client.client.get(&format!("/projects/{project_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListTaskAssignmentsForSpecificProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTaskAssignmentsForSpecificProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TaskAssignments> {
        let project_id = self.project_id;
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self
            .client
            .client
            .get(&format!("/projects/{project_id}/task_assignments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::TaskAssignment> {
        let project_id = self.project_id;
        let mut r = self
            .client
            .client
            .post(&format!("/projects/{project_id}/task_assignments"));
        r = r
            .json(
                json!(
                    { "task_id" : self.task_id, "is_active" : self.is_active, "billable"
                    : self.billable, "hourly_rate" : self.hourly_rate, "budget" : self
                    .budget }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::TaskAssignment> {
        let project_id = self.project_id;
        let task_assignment_id = self.task_assignment_id;
        let mut r = self
            .client
            .client
            .get(
                &format!("/projects/{project_id}/task_assignments/{task_assignment_id}"),
            );
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListUserAssignmentsForSpecificProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub user_id: i64,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListUserAssignmentsForSpecificProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UserAssignments> {
        let project_id = self.project_id;
        let user_id = self.user_id;
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self
            .client
            .client
            .get(&format!("/projects/{project_id}/user_assignments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::UserAssignment> {
        let project_id = self.project_id;
        let mut r = self
            .client
            .client
            .post(&format!("/projects/{project_id}/user_assignments"));
        r = r
            .json(
                json!(
                    { "user_id" : self.user_id, "is_active" : self.is_active,
                    "is_project_manager" : self.is_project_manager, "use_default_rates" :
                    self.use_default_rates, "hourly_rate" : self.hourly_rate, "budget" :
                    self.budget }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::UserAssignment> {
        let project_id = self.project_id;
        let user_assignment_id = self.user_assignment_id;
        let mut r = self
            .client
            .client
            .get(
                &format!("/projects/{project_id}/user_assignments/{user_assignment_id}"),
            );
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ExpenseCategoriesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ExpenseCategoriesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/expenses/categories"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ClientsExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ClientsExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/expenses/clients"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProjectsExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ProjectsExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/expenses/projects"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TeamExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> TeamExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/expenses/team"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProjectBudgetReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: i64,
    pub per_page: i64,
    pub is_active: bool,
}
impl<'a> ProjectBudgetReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProjectBudgetReportResults> {
        let page = self.page;
        let per_page = self.per_page;
        let is_active = self.is_active;
        let mut r = self.client.client.get(&format!("/reports/project_budget"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ClientsTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ClientsTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/time/clients"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ProjectsTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ProjectsTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/time/projects"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TasksReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> TasksReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/time/tasks"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct TeamTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> TeamTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/time/team"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct UninvoicedReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> UninvoicedReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UninvoicedReportResults> {
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/reports/uninvoiced"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListRolesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListRolesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Roles> {
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/roles"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateRoleRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
    pub user_ids: Vec<i64>,
}
impl<'a> CreateRoleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Role> {
        let mut r = self.client.client.post("/roles");
        r = r.json(json!({ "name" : self.name, "user_ids" : self.user_ids }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Role> {
        let role_id = self.role_id;
        let mut r = self.client.client.get(&format!("/roles/{role_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListTaskAssignmentsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTaskAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TaskAssignments> {
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/task_assignments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListTasksRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTasksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Tasks> {
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/tasks"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::Task> {
        let mut r = self.client.client.post("/tasks");
        r = r
            .json(
                json!(
                    { "name" : self.name, "billable_by_default" : self
                    .billable_by_default, "default_hourly_rate" : self
                    .default_hourly_rate, "is_default" : self.is_default, "is_active" :
                    self.is_active }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::Task> {
        let task_id = self.task_id;
        let mut r = self.client.client.get(&format!("/tasks/{task_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListTimeEntriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: i64,
    pub client_id: i64,
    pub project_id: i64,
    pub task_id: i64,
    pub external_reference_id: String,
    pub is_billed: bool,
    pub is_running: bool,
    pub updated_since: String,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTimeEntriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeEntries> {
        let user_id = self.user_id;
        let client_id = self.client_id;
        let project_id = self.project_id;
        let task_id = self.task_id;
        let external_reference_id = self.external_reference_id;
        let is_billed = self.is_billed;
        let is_running = self.is_running;
        let updated_since = self.updated_since;
        let from = self.from;
        let to = self.to;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/time_entries"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::TimeEntry> {
        let mut r = self.client.client.post("/time_entries");
        if let Some(ref external_reference) = self.external_reference {
            r = r.push_query("external_reference", &external_reference.to_string());
        }
        r = r
            .json(
                json!(
                    { "user_id" : self.user_id, "project_id" : self.project_id, "task_id"
                    : self.task_id, "spent_date" : self.spent_date, "started_time" : self
                    .started_time, "ended_time" : self.ended_time, "notes" : self.notes,
                    "external_reference" : self.external_reference, "hours" : self.hours
                    }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::TimeEntry> {
        let time_entry_id = self.time_entry_id;
        let mut r = self.client.client.get(&format!("/time_entries/{time_entry_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListUserAssignmentsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: i64,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListUserAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UserAssignments> {
        let user_id = self.user_id;
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/user_assignments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListUsersRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListUsersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Users> {
        let is_active = self.is_active;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/users"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
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
    pub async fn send(self) -> anyhow::Result<model::User> {
        let mut r = self.client.client.post("/users");
        r = r
            .json(
                json!(
                    { "first_name" : self.first_name, "last_name" : self.last_name,
                    "email" : self.email, "timezone" : self.timezone,
                    "has_access_to_all_future_projects" : self
                    .has_access_to_all_future_projects, "is_contractor" : self
                    .is_contractor, "is_active" : self.is_active, "weekly_capacity" :
                    self.weekly_capacity, "default_hourly_rate" : self
                    .default_hourly_rate, "cost_rate" : self.cost_rate, "roles" : self
                    .roles }
                ),
            );
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct RetrieveTheCurrentlyAuthenticatedUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
}
impl<'a> RetrieveTheCurrentlyAuthenticatedUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::User> {
        let mut r = self.client.client.get("/users/me");
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProjectAssignments> {
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/users/me/project_assignments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct RetrieveUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
}
impl<'a> RetrieveUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::User> {
        let user_id = self.user_id;
        let mut r = self.client.client.get(&format!("/users/{user_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListBillableRatesForSpecificUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListBillableRatesForSpecificUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BillableRates> {
        let user_id = self.user_id;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/users/{user_id}/billable_rates"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: f64,
    pub start_date: String,
}
impl<'a> CreateBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BillableRate> {
        let user_id = self.user_id;
        let mut r = self.client.client.post(&format!("/users/{user_id}/billable_rates"));
        r = r.json(json!({ "amount" : self.amount, "start_date" : self.start_date }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::BillableRate> {
        let user_id = self.user_id;
        let billable_rate_id = self.billable_rate_id;
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/billable_rates/{billable_rate_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListCostRatesForSpecificUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListCostRatesForSpecificUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CostRates> {
        let user_id = self.user_id;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self.client.client.get(&format!("/users/{user_id}/cost_rates"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateCostRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: f64,
    pub start_date: String,
}
impl<'a> CreateCostRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CostRate> {
        let user_id = self.user_id;
        let mut r = self.client.client.post(&format!("/users/{user_id}/cost_rates"));
        r = r.json(json!({ "amount" : self.amount, "start_date" : self.start_date }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
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
    pub async fn send(self) -> anyhow::Result<model::CostRate> {
        let user_id = self.user_id;
        let cost_rate_id = self.cost_rate_id;
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/cost_rates/{cost_rate_id}"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct ListActiveProjectAssignmentsRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListActiveProjectAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProjectAssignments> {
        let user_id = self.user_id;
        let updated_since = self.updated_since;
        let page = self.page;
        let per_page = self.per_page;
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/project_assignments"));
        r = r;
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e));
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
