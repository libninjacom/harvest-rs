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
        let mut r = self.client.client.get("/clients");
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "name", self.name
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
            );
        r = r
            .push_json(
                json! {
                    "address", self.address
                },
            );
        r = r
            .push_json(
                json! {
                    "currency", self.currency
                },
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
pub struct RetrieveClientRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub client_id: String,
}
impl<'a> RetrieveClientRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Client> {
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
    pub async fn send(self) -> anyhow::Result<model::Company> {
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
    pub client_id: i64,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListContactsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Contacts> {
        let mut r = self.client.client.get("/contacts");
        r = r.push_query("client_id", &self.client_id.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "client_id", self.client_id
                },
            );
        r = r
            .push_json(
                json! {
                    "title", self.title
                },
            );
        r = r
            .push_json(
                json! {
                    "first_name", self.first_name
                },
            );
        r = r
            .push_json(
                json! {
                    "last_name", self.last_name
                },
            );
        r = r
            .push_json(
                json! {
                    "email", self.email
                },
            );
        r = r
            .push_json(
                json! {
                    "phone_office", self.phone_office
                },
            );
        r = r
            .push_json(
                json! {
                    "phone_mobile", self.phone_mobile
                },
            );
        r = r
            .push_json(
                json! {
                    "fax", self.fax
                },
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
pub struct RetrieveContactRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub contact_id: String,
}
impl<'a> RetrieveContactRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Contact> {
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
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListEstimateItemCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateItemCategories> {
        let mut r = self.client.client.get("/estimate_item_categories");
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct CreateEstimateItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
}
impl<'a> CreateEstimateItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateItemCategory> {
        let mut r = self.client.client.post("/estimate_item_categories");
        r = r
            .push_json(
                json! {
                    "name", self.name
                },
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
pub struct RetrieveEstimateItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_item_category_id: String,
}
impl<'a> RetrieveEstimateItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateItemCategory> {
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
        let mut r = self.client.client.get("/estimates");
        r = r.push_query("client_id", &self.client_id.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("state", &self.state.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "client_id", self.client_id
                },
            );
        r = r
            .push_json(
                json! {
                    "number", self.number
                },
            );
        r = r
            .push_json(
                json! {
                    "purchase_order", self.purchase_order
                },
            );
        r = r
            .push_json(
                json! {
                    "tax", self.tax
                },
            );
        r = r
            .push_json(
                json! {
                    "tax2", self.tax_2
                },
            );
        r = r
            .push_json(
                json! {
                    "discount", self.discount
                },
            );
        r = r
            .push_json(
                json! {
                    "subject", self.subject
                },
            );
        r = r
            .push_json(
                json! {
                    "notes", self.notes
                },
            );
        r = r
            .push_json(
                json! {
                    "currency", self.currency
                },
            );
        r = r
            .push_json(
                json! {
                    "issue_date", self.issue_date
                },
            );
        r = r
            .push_json(
                json! {
                    "line_items", self.line_items
                },
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
pub struct RetrieveEstimateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub estimate_id: String,
}
impl<'a> RetrieveEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Estimate> {
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
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListMessagesForEstimateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::EstimateMessages> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/estimates/{estimate_id}/messages", estimate_id = self.estimate_id
                ),
            );
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/estimates/{estimate_id}/messages", estimate_id = self.estimate_id
                ),
            );
        r = r
            .push_json(
                json! {
                    "event_type", self.event_type
                },
            );
        r = r
            .push_json(
                json! {
                    "recipients", self.recipients
                },
            );
        r = r
            .push_json(
                json! {
                    "subject", self.subject
                },
            );
        r = r
            .push_json(
                json! {
                    "body", self.body
                },
            );
        r = r
            .push_json(
                json! {
                    "send_me_a_copy", self.send_me_a_copy
                },
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
pub struct ListExpenseCategoriesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListExpenseCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseCategories> {
        let mut r = self.client.client.get("/expense_categories");
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "name", self.name
                },
            );
        r = r
            .push_json(
                json! {
                    "unit_name", self.unit_name
                },
            );
        r = r
            .push_json(
                json! {
                    "unit_price", self.unit_price
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
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
pub struct RetrieveExpenseCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_category_id: String,
}
impl<'a> RetrieveExpenseCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseCategory> {
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
        let mut r = self.client.client.get("/expenses");
        r = r.push_query("user_id", &self.user_id.to_string());
        r = r.push_query("client_id", &self.client_id.to_string());
        r = r.push_query("project_id", &self.project_id.to_string());
        r = r.push_query("is_billed", &self.is_billed.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "user_id", self.user_id
                },
            );
        r = r
            .push_json(
                json! {
                    "project_id", self.project_id
                },
            );
        r = r
            .push_json(
                json! {
                    "expense_category_id", self.expense_category_id
                },
            );
        r = r
            .push_json(
                json! {
                    "spent_date", self.spent_date
                },
            );
        r = r
            .push_json(
                json! {
                    "units", self.units
                },
            );
        r = r
            .push_json(
                json! {
                    "total_cost", self.total_cost
                },
            );
        r = r
            .push_json(
                json! {
                    "notes", self.notes
                },
            );
        r = r
            .push_json(
                json! {
                    "billable", self.billable
                },
            );
        r = r
            .push_json(
                json! {
                    "receipt", self.receipt
                },
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
pub struct RetrieveExpenseRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub expense_id: String,
}
impl<'a> RetrieveExpenseRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Expense> {
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
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListInvoiceItemCategoriesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceItemCategories> {
        let mut r = self.client.client.get("/invoice_item_categories");
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct CreateInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
}
impl<'a> CreateInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceItemCategory> {
        let mut r = self.client.client.post("/invoice_item_categories");
        r = r
            .push_json(
                json! {
                    "name", self.name
                },
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
pub struct RetrieveInvoiceItemCategoryRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_item_category_id: String,
}
impl<'a> RetrieveInvoiceItemCategoryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceItemCategory> {
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
        let mut r = self.client.client.get("/invoices");
        r = r.push_query("client_id", &self.client_id.to_string());
        r = r.push_query("project_id", &self.project_id.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("state", &self.state.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        r = r
            .push_json(
                json! {
                    "client_id", self.client_id
                },
            );
        r = r
            .push_json(
                json! {
                    "retainer_id", self.retainer_id
                },
            );
        r = r
            .push_json(
                json! {
                    "estimate_id", self.estimate_id
                },
            );
        r = r
            .push_json(
                json! {
                    "number", self.number
                },
            );
        r = r
            .push_json(
                json! {
                    "purchase_order", self.purchase_order
                },
            );
        r = r
            .push_json(
                json! {
                    "tax", self.tax
                },
            );
        r = r
            .push_json(
                json! {
                    "tax2", self.tax_2
                },
            );
        r = r
            .push_json(
                json! {
                    "discount", self.discount
                },
            );
        r = r
            .push_json(
                json! {
                    "subject", self.subject
                },
            );
        r = r
            .push_json(
                json! {
                    "notes", self.notes
                },
            );
        r = r
            .push_json(
                json! {
                    "currency", self.currency
                },
            );
        r = r
            .push_json(
                json! {
                    "issue_date", self.issue_date
                },
            );
        r = r
            .push_json(
                json! {
                    "due_date", self.due_date
                },
            );
        r = r
            .push_json(
                json! {
                    "payment_term", self.payment_term
                },
            );
        if let Some(line_items_import) = self.line_items_import {
            r = r
                .push_json(
                    json! {
                        "line_items_import", self.line_items_import
                    },
                );
        }
        r = r
            .push_json(
                json! {
                    "line_items", self.line_items
                },
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
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListMessagesForInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoiceMessages> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/invoices/{invoice_id}/messages", invoice_id = self.invoice_id),
            );
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/messages", invoice_id = self.invoice_id),
            );
        r = r
            .push_json(
                json! {
                    "event_type", self.event_type
                },
            );
        r = r
            .push_json(
                json! {
                    "recipients", self.recipients
                },
            );
        r = r
            .push_json(
                json! {
                    "subject", self.subject
                },
            );
        r = r
            .push_json(
                json! {
                    "body", self.body
                },
            );
        r = r
            .push_json(
                json! {
                    "include_link_to_client_invoice", self.include_link_to_client_invoice
                },
            );
        r = r
            .push_json(
                json! {
                    "attach_pdf", self.attach_pdf
                },
            );
        r = r
            .push_json(
                json! {
                    "send_me_a_copy", self.send_me_a_copy
                },
            );
        r = r
            .push_json(
                json! {
                    "thank_you", self.thank_you
                },
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
pub struct ListPaymentsForInvoiceRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub invoice_id: String,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListPaymentsForInvoiceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::InvoicePayments> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/invoices/{invoice_id}/payments", invoice_id = self.invoice_id),
            );
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        let mut r = self
            .client
            .client
            .post(
                &format!("/invoices/{invoice_id}/payments", invoice_id = self.invoice_id),
            );
        r = r
            .push_json(
                json! {
                    "amount", self.amount
                },
            );
        r = r
            .push_json(
                json! {
                    "paid_at", self.paid_at
                },
            );
        r = r
            .push_json(
                json! {
                    "paid_date", self.paid_date
                },
            );
        r = r
            .push_json(
                json! {
                    "notes", self.notes
                },
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
        let mut r = self.client.client.get("/projects");
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("client_id", &self.client_id.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "client_id", self.client_id
                },
            );
        r = r
            .push_json(
                json! {
                    "name", self.name
                },
            );
        r = r
            .push_json(
                json! {
                    "code", self.code
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
            );
        r = r
            .push_json(
                json! {
                    "is_billable", self.is_billable
                },
            );
        r = r
            .push_json(
                json! {
                    "is_fixed_fee", self.is_fixed_fee
                },
            );
        r = r
            .push_json(
                json! {
                    "bill_by", self.bill_by
                },
            );
        r = r
            .push_json(
                json! {
                    "hourly_rate", self.hourly_rate
                },
            );
        r = r
            .push_json(
                json! {
                    "budget", self.budget
                },
            );
        r = r
            .push_json(
                json! {
                    "budget_by", self.budget_by
                },
            );
        r = r
            .push_json(
                json! {
                    "budget_is_monthly", self.budget_is_monthly
                },
            );
        r = r
            .push_json(
                json! {
                    "notify_when_over_budget", self.notify_when_over_budget
                },
            );
        r = r
            .push_json(
                json! {
                    "over_budget_notification_percentage", self
                    .over_budget_notification_percentage
                },
            );
        r = r
            .push_json(
                json! {
                    "show_budget_to_all", self.show_budget_to_all
                },
            );
        r = r
            .push_json(
                json! {
                    "cost_budget", self.cost_budget
                },
            );
        r = r
            .push_json(
                json! {
                    "cost_budget_include_expenses", self.cost_budget_include_expenses
                },
            );
        r = r
            .push_json(
                json! {
                    "fee", self.fee
                },
            );
        r = r
            .push_json(
                json! {
                    "notes", self.notes
                },
            );
        r = r
            .push_json(
                json! {
                    "starts_on", self.starts_on
                },
            );
        r = r
            .push_json(
                json! {
                    "ends_on", self.ends_on
                },
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
pub struct RetrieveProjectRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
}
impl<'a> RetrieveProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Project> {
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
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTaskAssignmentsForSpecificProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TaskAssignments> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/projects/{project_id}/task_assignments", project_id = self
                    .project_id
                ),
            );
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/projects/{project_id}/task_assignments", project_id = self
                    .project_id
                ),
            );
        r = r
            .push_json(
                json! {
                    "task_id", self.task_id
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
            );
        r = r
            .push_json(
                json! {
                    "billable", self.billable
                },
            );
        r = r
            .push_json(
                json! {
                    "hourly_rate", self.hourly_rate
                },
            );
        r = r
            .push_json(
                json! {
                    "budget", self.budget
                },
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
pub struct RetrieveTaskAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub task_assignment_id: String,
}
impl<'a> RetrieveTaskAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TaskAssignment> {
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
    pub user_id: i64,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListUserAssignmentsForSpecificProjectRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UserAssignments> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/projects/{project_id}/user_assignments", project_id = self
                    .project_id
                ),
            );
        r = r.push_query("user_id", &self.user_id.to_string());
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/projects/{project_id}/user_assignments", project_id = self
                    .project_id
                ),
            );
        r = r
            .push_json(
                json! {
                    "user_id", self.user_id
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
            );
        r = r
            .push_json(
                json! {
                    "is_project_manager", self.is_project_manager
                },
            );
        r = r
            .push_json(
                json! {
                    "use_default_rates", self.use_default_rates
                },
            );
        r = r
            .push_json(
                json! {
                    "hourly_rate", self.hourly_rate
                },
            );
        r = r
            .push_json(
                json! {
                    "budget", self.budget
                },
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
pub struct RetrieveUserAssignmentRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub project_id: String,
    pub user_assignment_id: String,
}
impl<'a> RetrieveUserAssignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UserAssignment> {
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
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ExpenseCategoriesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/categories");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ClientsExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ClientsExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/clients");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ProjectsExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ProjectsExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/projects");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct TeamExpensesReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> TeamExpensesReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ExpenseReportsResults> {
        let mut r = self.client.client.get("/reports/expenses/team");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ProjectBudgetReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: i64,
    pub per_page: i64,
    pub is_active: bool,
}
impl<'a> ProjectBudgetReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProjectBudgetReportResults> {
        let mut r = self.client.client.get("/reports/project_budget");
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
        r = r.push_query("is_active", &self.is_active.to_string());
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
pub struct ClientsTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ClientsTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/clients");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ProjectsTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ProjectsTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/projects");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct TasksReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> TasksReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/tasks");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct TeamTimeReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> TeamTimeReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TimeReportsResults> {
        let mut r = self.client.client.get("/reports/time/team");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct UninvoicedReportRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub from: String,
    pub to: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> UninvoicedReportRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UninvoicedReportResults> {
        let mut r = self.client.client.get("/reports/uninvoiced");
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ListRolesRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListRolesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Roles> {
        let mut r = self.client.client.get("/roles");
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct CreateRoleRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub name: String,
    pub user_ids: Vec<i64>,
}
impl<'a> CreateRoleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Role> {
        let mut r = self.client.client.post("/roles");
        r = r
            .push_json(
                json! {
                    "name", self.name
                },
            );
        r = r
            .push_json(
                json! {
                    "user_ids", self.user_ids
                },
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
pub struct RetrieveRoleRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub role_id: String,
}
impl<'a> RetrieveRoleRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Role> {
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
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTaskAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::TaskAssignments> {
        let mut r = self.client.client.get("/task_assignments");
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ListTasksRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListTasksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Tasks> {
        let mut r = self.client.client.get("/tasks");
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "name", self.name
                },
            );
        r = r
            .push_json(
                json! {
                    "billable_by_default", self.billable_by_default
                },
            );
        r = r
            .push_json(
                json! {
                    "default_hourly_rate", self.default_hourly_rate
                },
            );
        r = r
            .push_json(
                json! {
                    "is_default", self.is_default
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
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
pub struct RetrieveTaskRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub task_id: String,
}
impl<'a> RetrieveTaskRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Task> {
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
        let mut r = self.client.client.get("/time_entries");
        r = r.push_query("user_id", &self.user_id.to_string());
        r = r.push_query("client_id", &self.client_id.to_string());
        r = r.push_query("project_id", &self.project_id.to_string());
        r = r.push_query("task_id", &self.task_id.to_string());
        r = r
            .push_query(
                "external_reference_id",
                &self.external_reference_id.to_string(),
            );
        r = r.push_query("is_billed", &self.is_billed.to_string());
        r = r.push_query("is_running", &self.is_running.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("from", &self.from.to_string());
        r = r.push_query("to", &self.to.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
        r = r
            .push_json(
                json! {
                    "user_id", self.user_id
                },
            );
        r = r
            .push_json(
                json! {
                    "project_id", self.project_id
                },
            );
        r = r
            .push_json(
                json! {
                    "task_id", self.task_id
                },
            );
        r = r
            .push_json(
                json! {
                    "spent_date", self.spent_date
                },
            );
        r = r
            .push_json(
                json! {
                    "started_time", self.started_time
                },
            );
        r = r
            .push_json(
                json! {
                    "ended_time", self.ended_time
                },
            );
        r = r
            .push_json(
                json! {
                    "notes", self.notes
                },
            );
        if let Some(external_reference) = self.external_reference {
            r = r
                .push_json(
                    json! {
                        "external_reference", self.external_reference
                    },
                );
        }
        r = r
            .push_json(
                json! {
                    "hours", self.hours
                },
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
    pub user_id: i64,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListUserAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::UserAssignments> {
        let mut r = self.client.client.get("/user_assignments");
        r = r.push_query("user_id", &self.user_id.to_string());
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct ListUsersRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub is_active: bool,
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListUsersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::Users> {
        let mut r = self.client.client.get("/users");
        r = r.push_query("is_active", &self.is_active.to_string());
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
            .push_json(
                json! {
                    "first_name", self.first_name
                },
            );
        r = r
            .push_json(
                json! {
                    "last_name", self.last_name
                },
            );
        r = r
            .push_json(
                json! {
                    "email", self.email
                },
            );
        r = r
            .push_json(
                json! {
                    "timezone", self.timezone
                },
            );
        r = r
            .push_json(
                json! {
                    "has_access_to_all_future_projects", self
                    .has_access_to_all_future_projects
                },
            );
        r = r
            .push_json(
                json! {
                    "is_contractor", self.is_contractor
                },
            );
        r = r
            .push_json(
                json! {
                    "is_active", self.is_active
                },
            );
        r = r
            .push_json(
                json! {
                    "weekly_capacity", self.weekly_capacity
                },
            );
        r = r
            .push_json(
                json! {
                    "default_hourly_rate", self.default_hourly_rate
                },
            );
        r = r
            .push_json(
                json! {
                    "cost_rate", self.cost_rate
                },
            );
        r = r
            .push_json(
                json! {
                    "roles", self.roles
                },
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
pub struct RetrieveTheCurrentlyAuthenticatedUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
}
impl<'a> RetrieveTheCurrentlyAuthenticatedUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::User> {
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
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProjectAssignments> {
        let mut r = self.client.client.get("/users/me/project_assignments");
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct RetrieveUserRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
}
impl<'a> RetrieveUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::User> {
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
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListBillableRatesForSpecificUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BillableRates> {
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/billable_rates", user_id = self.user_id));
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct CreateBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: f64,
    pub start_date: String,
}
impl<'a> CreateBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BillableRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/users/{user_id}/billable_rates", user_id = self.user_id));
        r = r
            .push_json(
                json! {
                    "amount", self.amount
                },
            );
        r = r
            .push_json(
                json! {
                    "start_date", self.start_date
                },
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
pub struct RetrieveBillableRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub billable_rate_id: String,
}
impl<'a> RetrieveBillableRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::BillableRate> {
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
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListCostRatesForSpecificUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CostRates> {
        let mut r = self
            .client
            .client
            .get(&format!("/users/{user_id}/cost_rates", user_id = self.user_id));
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
pub struct CreateCostRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub amount: f64,
    pub start_date: String,
}
impl<'a> CreateCostRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CostRate> {
        let mut r = self
            .client
            .client
            .post(&format!("/users/{user_id}/cost_rates", user_id = self.user_id));
        r = r
            .push_json(
                json! {
                    "amount", self.amount
                },
            );
        r = r
            .push_json(
                json! {
                    "start_date", self.start_date
                },
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
pub struct RetrieveCostRateRequest<'a> {
    pub(crate) client: &'a HarvestClient,
    pub user_id: String,
    pub cost_rate_id: String,
}
impl<'a> RetrieveCostRateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::CostRate> {
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
    pub updated_since: String,
    pub page: i64,
    pub per_page: i64,
}
impl<'a> ListActiveProjectAssignmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<model::ProjectAssignments> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/users/{user_id}/project_assignments", user_id = self.user_id),
            );
        r = r.push_query("updated_since", &self.updated_since.to_string());
        r = r.push_query("page", &self.page.to_string());
        r = r.push_query("per_page", &self.per_page.to_string());
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
