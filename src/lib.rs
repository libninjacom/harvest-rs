//! [`HarvestClient`](struct.HarvestClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
use crate::model::*;

pub struct HarvestClient {
    pub(crate) client: httpclient::Client,
    authentication: HarvestAuthentication,
}
impl HarvestClient {
    pub fn from_env() -> Self {
        let url = "https://api.harvestapp.com/v2".to_string();
        Self {
            client: httpclient::Client::new(Some(url)),
            authentication: HarvestAuthentication::from_env(),
        }
    }
}
impl HarvestClient {
    pub fn new(url: &str, authentication: HarvestAuthentication) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: HarvestAuthentication) -> Self {
        self.authentication = authentication;
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        match &self.authentication {
            HarvestAuthentication::BearerAuth { bearer_auth, account_auth } => {
                r = r.header("Authorization", bearer_auth);
                r = r.header("Harvest-Account-Id", account_auth);
            }
        }
        r
    }
    pub fn with_middleware<M: httpclient::Middleware + 'static>(
        mut self,
        middleware: M,
    ) -> Self {
        self.client = self.client.with_middleware(middleware);
        self
    }
    /**List all clients

Returns a list of your clients. The clients are returned sorted by creation date, with the most recently created clients appearing first.

The response contains an object with a clients property that contains an array of up to per_page clients. Each entry in the array is a separate client object. If no more clients are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your clients.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/clients/#list-all-clients>.*/
    pub fn list_clients(&self) -> request::ListClientsRequest {
        request::ListClientsRequest {
            client: &self,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a client

Creates a new client object. Returns a client object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/clients/#create-a-client>.*/
    pub fn create_client(&self) -> request::CreateClientRequest {
        request::CreateClientRequest {
            client: &self,
            name: None,
            is_active: None,
            address: None,
            currency: None,
        }
    }
    /**Retrieve a client

Retrieves the client with the given ID. Returns a client object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/clients/#retrieve-a-client>.*/
    pub fn retrieve_client(&self, client_id: &str) -> request::RetrieveClientRequest {
        request::RetrieveClientRequest {
            client: &self,
            client_id: client_id.to_owned(),
        }
    }
    /**Delete a client

Delete a client. Deleting a client is only possible if it has no projects, invoices, or estimates associated with it. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/clients/#delete-a-client>.*/
    pub fn delete_client(&self, client_id: &str) -> request::DeleteClientRequest {
        request::DeleteClientRequest {
            client: &self,
            client_id: client_id.to_owned(),
        }
    }
    /**Update a client

Updates the specific client by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a client object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/clients/#update-a-client>.*/
    pub fn update_client(&self, client_id: &str) -> request::UpdateClientRequest {
        request::UpdateClientRequest {
            client: &self,
            client_id: client_id.to_owned(),
            name: None,
            is_active: None,
            address: None,
            currency: None,
        }
    }
    /**Retrieve a company

Retrieves the company for the currently authenticated user. Returns a
company object and a 200 OK response code.

See endpoint docs at <https://help.getharvest.com/api-v2/company-api/company/company/#retrieve-a-company>.*/
    pub fn retrieve_company(&self) -> request::RetrieveCompanyRequest {
        request::RetrieveCompanyRequest {
            client: &self,
        }
    }
    /**Update a company

Updates the company setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a company object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/company-api/company/company/#update-a-company>.*/
    pub fn update_company(&self) -> request::UpdateCompanyRequest {
        request::UpdateCompanyRequest {
            client: &self,
            wants_timestamp_timers: None,
            weekly_capacity: None,
        }
    }
    /**List all contacts

Returns a list of your contacts. The contacts are returned sorted by creation date, with the most recently created contacts appearing first.

The response contains an object with a contacts property that contains an array of up to per_page contacts. Each entry in the array is a separate contact object. If no more contacts are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your contacts.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#list-all-contacts>.*/
    pub fn list_contacts(&self) -> request::ListContactsRequest {
        request::ListContactsRequest {
            client: &self,
            client_id: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a contact

Creates a new contact object. Returns a contact object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#create-a-contact>.*/
    pub fn create_contact(&self) -> request::CreateContactRequest {
        request::CreateContactRequest {
            client: &self,
            client_id: None,
            title: None,
            first_name: None,
            last_name: None,
            email: None,
            phone_office: None,
            phone_mobile: None,
            fax: None,
        }
    }
    /**Retrieve a contact

Retrieves the contact with the given ID. Returns a contact object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#retrieve-a-contact>.*/
    pub fn retrieve_contact(&self, contact_id: &str) -> request::RetrieveContactRequest {
        request::RetrieveContactRequest {
            client: &self,
            contact_id: contact_id.to_owned(),
        }
    }
    /**Delete a contact

Delete a contact. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#delete-a-contact>.*/
    pub fn delete_contact(&self, contact_id: &str) -> request::DeleteContactRequest {
        request::DeleteContactRequest {
            client: &self,
            contact_id: contact_id.to_owned(),
        }
    }
    /**Update a contact

Updates the specific contact by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a contact object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#update-a-contact>.*/
    pub fn update_contact(&self, contact_id: &str) -> request::UpdateContactRequest {
        request::UpdateContactRequest {
            client: &self,
            contact_id: contact_id.to_owned(),
            client_id: None,
            title: None,
            first_name: None,
            last_name: None,
            email: None,
            phone_office: None,
            phone_mobile: None,
            fax: None,
        }
    }
    /**List all estimate item categories

Returns a list of your estimate item categories. The estimate item categories are returned sorted by creation date, with the most recently created estimate item categories appearing first.

The response contains an object with a estimate_item_categories property that contains an array of up to per_page estimate item categories. Each entry in the array is a separate estimate item category object. If no more estimate item categories are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your estimate item categories.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#list-all-estimate-item-categories>.*/
    pub fn list_estimate_item_categories(
        &self,
    ) -> request::ListEstimateItemCategoriesRequest {
        request::ListEstimateItemCategoriesRequest {
            client: &self,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an estimate item category

Creates a new estimate item category object. Returns an estimate item category object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#create-an-estimate-item-category>.*/
    pub fn create_estimate_item_category(
        &self,
    ) -> request::CreateEstimateItemCategoryRequest {
        request::CreateEstimateItemCategoryRequest {
            client: &self,
            name: None,
        }
    }
    /**Retrieve an estimate item category

Retrieves the estimate item category with the given ID. Returns an estimate item category object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#retrieve-an-estimate-item-category>.*/
    pub fn retrieve_estimate_item_category(
        &self,
        estimate_item_category_id: &str,
    ) -> request::RetrieveEstimateItemCategoryRequest {
        request::RetrieveEstimateItemCategoryRequest {
            client: &self,
            estimate_item_category_id: estimate_item_category_id.to_owned(),
        }
    }
    /**Delete an estimate item category

Delete an estimate item category. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#delete-an-estimate-item-category>.*/
    pub fn delete_estimate_item_category(
        &self,
        estimate_item_category_id: &str,
    ) -> request::DeleteEstimateItemCategoryRequest {
        request::DeleteEstimateItemCategoryRequest {
            client: &self,
            estimate_item_category_id: estimate_item_category_id.to_owned(),
        }
    }
    /**Update an estimate item category

Updates the specific estimate item category by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns an estimate item category object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#update-an-estimate-item-category>.*/
    pub fn update_estimate_item_category(
        &self,
        estimate_item_category_id: &str,
    ) -> request::UpdateEstimateItemCategoryRequest {
        request::UpdateEstimateItemCategoryRequest {
            client: &self,
            estimate_item_category_id: estimate_item_category_id.to_owned(),
            name: None,
        }
    }
    /**List all estimates

Returns a list of your estimates. The estimates are returned sorted by issue date, with the most recently issued estimates appearing first.

The response contains an object with a estimates property that contains an array of up to per_page estimates. Each entry in the array is a separate estimate object. If no more estimates are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your estimates.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#list-all-estimates>.*/
    pub fn list_estimates(&self) -> request::ListEstimatesRequest {
        request::ListEstimatesRequest {
            client: &self,
            client_id: None,
            updated_since: None,
            from: None,
            to: None,
            state: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an estimate

Creates a new estimate object. Returns an estimate object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#create-an-estimate>.*/
    pub fn create_estimate(&self) -> request::CreateEstimateRequest {
        request::CreateEstimateRequest {
            client: &self,
            client_id: None,
            number: None,
            purchase_order: None,
            tax: None,
            tax2: None,
            discount: None,
            subject: None,
            notes: None,
            currency: None,
            issue_date: None,
            line_items: None,
        }
    }
    /**Retrieve an estimate

Retrieves the estimate with the given ID. Returns an estimate object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#retrieve-an-estimate>.*/
    pub fn retrieve_estimate(
        &self,
        estimate_id: &str,
    ) -> request::RetrieveEstimateRequest {
        request::RetrieveEstimateRequest {
            client: &self,
            estimate_id: estimate_id.to_owned(),
        }
    }
    /**Delete an estimate

Delete an estimate. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#delete-an-estimate>.*/
    pub fn delete_estimate(&self, estimate_id: &str) -> request::DeleteEstimateRequest {
        request::DeleteEstimateRequest {
            client: &self,
            estimate_id: estimate_id.to_owned(),
        }
    }
    /**Update an estimate

Updates the specific estimate by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns an estimate object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#update-an-estimate>.*/
    pub fn update_estimate(&self, estimate_id: &str) -> request::UpdateEstimateRequest {
        request::UpdateEstimateRequest {
            client: &self,
            estimate_id: estimate_id.to_owned(),
            client_id: None,
            number: None,
            purchase_order: None,
            tax: None,
            tax2: None,
            discount: None,
            subject: None,
            notes: None,
            currency: None,
            issue_date: None,
            line_items: None,
        }
    }
    /**List all messages for an estimate

Returns a list of messages associated with a given estimate. The estimate messages are returned sorted by creation date, with the most recently created messages appearing first.

The response contains an object with an estimate_messages property that contains an array of up to per_page messages. Each entry in the array is a separate message object. If no more messages are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your messages.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-messages/#list-all-messages-for-an-estimate>.*/
    pub fn list_messages_for_estimate(
        &self,
        estimate_id: &str,
    ) -> request::ListMessagesForEstimateRequest {
        request::ListMessagesForEstimateRequest {
            client: &self,
            estimate_id: estimate_id.to_owned(),
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an estimate message or change estimate status

Creates a new estimate message object. Returns an estimate message object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-messages/#create-an-estimate-message>.*/
    pub fn create_estimate_message(
        &self,
        estimate_id: &str,
    ) -> request::CreateEstimateMessageRequest {
        request::CreateEstimateMessageRequest {
            client: &self,
            estimate_id: estimate_id.to_owned(),
            event_type: None,
            recipients: None,
            subject: None,
            body: None,
            send_me_a_copy: None,
        }
    }
    /**Delete an estimate message

Delete an estimate message. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-messages/#delete-an-estimate-message>.*/
    pub fn delete_estimate_message(
        &self,
        estimate_id: &str,
        message_id: &str,
    ) -> request::DeleteEstimateMessageRequest {
        request::DeleteEstimateMessageRequest {
            client: &self,
            estimate_id: estimate_id.to_owned(),
            message_id: message_id.to_owned(),
        }
    }
    /**List all expense categories

Returns a list of your expense categories. The expense categories are returned sorted by creation date, with the most recently created expense categories appearing first.

The response contains an object with a expense_categories property that contains an array of up to per_page expense categories. Each entry in the array is a separate expense category object. If no more expense categories are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your expense categories.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#list-all-expense-categories>.*/
    pub fn list_expense_categories(&self) -> request::ListExpenseCategoriesRequest {
        request::ListExpenseCategoriesRequest {
            client: &self,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an expense category

Creates a new expense category object. Returns an expense category object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#create-an-expense-category>.*/
    pub fn create_expense_category(&self) -> request::CreateExpenseCategoryRequest {
        request::CreateExpenseCategoryRequest {
            client: &self,
            name: None,
            unit_name: None,
            unit_price: None,
            is_active: None,
        }
    }
    /**Retrieve an expense category

Retrieves the expense category with the given ID. Returns an expense category object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#retrieve-an-expense-category>.*/
    pub fn retrieve_expense_category(
        &self,
        expense_category_id: &str,
    ) -> request::RetrieveExpenseCategoryRequest {
        request::RetrieveExpenseCategoryRequest {
            client: &self,
            expense_category_id: expense_category_id.to_owned(),
        }
    }
    /**Delete an expense category

Delete an expense category. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#delete-an-expense-category>.*/
    pub fn delete_expense_category(
        &self,
        expense_category_id: &str,
    ) -> request::DeleteExpenseCategoryRequest {
        request::DeleteExpenseCategoryRequest {
            client: &self,
            expense_category_id: expense_category_id.to_owned(),
        }
    }
    /**Update an expense category

Updates the specific expense category by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns an expense category object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#update-an-expense-category>.*/
    pub fn update_expense_category(
        &self,
        expense_category_id: &str,
    ) -> request::UpdateExpenseCategoryRequest {
        request::UpdateExpenseCategoryRequest {
            client: &self,
            expense_category_id: expense_category_id.to_owned(),
            name: None,
            unit_name: None,
            unit_price: None,
            is_active: None,
        }
    }
    /**List all expenses

Returns a list of your expenses. If accessing this endpoint as an Administrator, all expenses in the account will be returned. If accessing this endpoint as a Manager, all expenses for assigned teammates and managed projects will be returned. The expenses are returned sorted by the spent_at date, with the most recent expenses appearing first.

The response contains an object with a expenses property that contains an array of up to per_page expenses. Each entry in the array is a separate expense object. If no more expenses are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your expenses.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#list-all-expenses>.*/
    pub fn list_expenses(&self) -> request::ListExpensesRequest {
        request::ListExpensesRequest {
            client: &self,
            user_id: None,
            client_id: None,
            project_id: None,
            is_billed: None,
            updated_since: None,
            from: None,
            to: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an expense

Creates a new expense object. Returns an expense object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#create-an-expense>.*/
    pub fn create_expense(&self) -> request::CreateExpenseRequest {
        request::CreateExpenseRequest {
            client: &self,
            user_id: None,
            project_id: None,
            expense_category_id: None,
            spent_date: None,
            units: None,
            total_cost: None,
            notes: None,
            billable: None,
            receipt: None,
        }
    }
    /**Retrieve an expense

Retrieves the expense with the given ID. Returns an expense object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#retrieve-an-expense>.*/
    pub fn retrieve_expense(&self, expense_id: &str) -> request::RetrieveExpenseRequest {
        request::RetrieveExpenseRequest {
            client: &self,
            expense_id: expense_id.to_owned(),
        }
    }
    /**Delete an expense

Delete an expense. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#delete-an-expense>.*/
    pub fn delete_expense(&self, expense_id: &str) -> request::DeleteExpenseRequest {
        request::DeleteExpenseRequest {
            client: &self,
            expense_id: expense_id.to_owned(),
        }
    }
    /**Update an expense

Updates the specific expense by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns an expense object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#update-an-expense>.*/
    pub fn update_expense(&self, expense_id: &str) -> request::UpdateExpenseRequest {
        request::UpdateExpenseRequest {
            client: &self,
            expense_id: expense_id.to_owned(),
            project_id: None,
            expense_category_id: None,
            spent_date: None,
            units: None,
            total_cost: None,
            notes: None,
            billable: None,
            receipt: None,
            delete_receipt: None,
        }
    }
    /**List all invoice item categories

Returns a list of your invoice item categories. The invoice item categories are returned sorted by creation date, with the most recently created invoice item categories appearing first.

The response contains an object with a invoice_item_categories property that contains an array of up to per_page invoice item categories. Each entry in the array is a separate invoice item category object. If no more invoice item categories are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your invoice item categories.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#list-all-invoice-item-categories>.*/
    pub fn list_invoice_item_categories(
        &self,
    ) -> request::ListInvoiceItemCategoriesRequest {
        request::ListInvoiceItemCategoriesRequest {
            client: &self,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an invoice item category

Creates a new invoice item category object. Returns an invoice item category object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#create-an-invoice-item-category>.*/
    pub fn create_invoice_item_category(
        &self,
    ) -> request::CreateInvoiceItemCategoryRequest {
        request::CreateInvoiceItemCategoryRequest {
            client: &self,
            name: None,
        }
    }
    /**Retrieve an invoice item category

Retrieves the invoice item category with the given ID. Returns an invoice item category object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#retrieve-an-invoice-item-category>.*/
    pub fn retrieve_invoice_item_category(
        &self,
        invoice_item_category_id: &str,
    ) -> request::RetrieveInvoiceItemCategoryRequest {
        request::RetrieveInvoiceItemCategoryRequest {
            client: &self,
            invoice_item_category_id: invoice_item_category_id.to_owned(),
        }
    }
    /**Delete an invoice item category

Delete an invoice item category. Deleting an invoice item category is only possible if use_as_service and use_as_expense are both false. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#delete-an-invoice-item-category>.*/
    pub fn delete_invoice_item_category(
        &self,
        invoice_item_category_id: &str,
    ) -> request::DeleteInvoiceItemCategoryRequest {
        request::DeleteInvoiceItemCategoryRequest {
            client: &self,
            invoice_item_category_id: invoice_item_category_id.to_owned(),
        }
    }
    /**Update an invoice item category

Updates the specific invoice item category by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns an invoice item category object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#update-an-invoice-item-category>.*/
    pub fn update_invoice_item_category(
        &self,
        invoice_item_category_id: &str,
    ) -> request::UpdateInvoiceItemCategoryRequest {
        request::UpdateInvoiceItemCategoryRequest {
            client: &self,
            invoice_item_category_id: invoice_item_category_id.to_owned(),
            name: None,
        }
    }
    /**List all invoices

Returns a list of your invoices. The invoices are returned sorted by issue date, with the most recently issued invoices appearing first.

The response contains an object with a invoices property that contains an array of up to per_page invoices. Each entry in the array is a separate invoice object. If no more invoices are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your invoices.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#list-all-invoices>.*/
    pub fn list_invoices(&self) -> request::ListInvoicesRequest {
        request::ListInvoicesRequest {
            client: &self,
            client_id: None,
            project_id: None,
            updated_since: None,
            from: None,
            to: None,
            state: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an invoice

Creates a new invoice object. Returns an invoice object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#create-a-free-form-invoice>.*/
    pub fn create_invoice(&self) -> request::CreateInvoiceRequest {
        request::CreateInvoiceRequest {
            client: &self,
            client_id: None,
            retainer_id: None,
            estimate_id: None,
            number: None,
            purchase_order: None,
            tax: None,
            tax2: None,
            discount: None,
            subject: None,
            notes: None,
            currency: None,
            issue_date: None,
            due_date: None,
            payment_term: None,
            line_items_import: None,
            line_items: None,
        }
    }
    /**Retrieve an invoice

Retrieves the invoice with the given ID. Returns an invoice object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#retrieve-an-invoice>.*/
    pub fn retrieve_invoice(&self, invoice_id: &str) -> request::RetrieveInvoiceRequest {
        request::RetrieveInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Delete an invoice

Delete an invoice. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#delete-an-invoice>.*/
    pub fn delete_invoice(&self, invoice_id: &str) -> request::DeleteInvoiceRequest {
        request::DeleteInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
        }
    }
    /**Update an invoice

Updates the specific invoice by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns an invoice object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#update-an-invoice>.*/
    pub fn update_invoice(&self, invoice_id: &str) -> request::UpdateInvoiceRequest {
        request::UpdateInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            client_id: None,
            retainer_id: None,
            estimate_id: None,
            number: None,
            purchase_order: None,
            tax: None,
            tax2: None,
            discount: None,
            subject: None,
            notes: None,
            currency: None,
            issue_date: None,
            due_date: None,
            payment_term: None,
            line_items: None,
        }
    }
    /**List all messages for an invoice

Returns a list of messages associated with a given invoice. The invoice messages are returned sorted by creation date, with the most recently created messages appearing first.

The response contains an object with an invoice_messages property that contains an array of up to per_page messages. Each entry in the array is a separate message object. If no more messages are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your messages.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-messages/#list-all-messages-for-an-invoice>.*/
    pub fn list_messages_for_invoice(
        &self,
        invoice_id: &str,
    ) -> request::ListMessagesForInvoiceRequest {
        request::ListMessagesForInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an invoice message or change invoice status

Creates a new invoice message object. Returns an invoice message object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-messages/#create-an-invoice-message>.*/
    pub fn create_invoice_message(
        &self,
        invoice_id: &str,
    ) -> request::CreateInvoiceMessageRequest {
        request::CreateInvoiceMessageRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            event_type: None,
            recipients: None,
            subject: None,
            body: None,
            include_link_to_client_invoice: None,
            attach_pdf: None,
            send_me_a_copy: None,
            thank_you: None,
        }
    }
    /**Delete an invoice message

Delete an invoice message. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-messages/#delete-an-invoice-message>.*/
    pub fn delete_invoice_message(
        &self,
        invoice_id: &str,
        message_id: &str,
    ) -> request::DeleteInvoiceMessageRequest {
        request::DeleteInvoiceMessageRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            message_id: message_id.to_owned(),
        }
    }
    /**List all payments for an invoice

Returns a list of payments associate with a given invoice. The payments are returned sorted by creation date, with the most recently created payments appearing first.

The response contains an object with an invoice_payments property that contains an array of up to per_page payments. Each entry in the array is a separate payment object. If no more payments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your payments.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-payments/#list-all-payments-for-an-invoice>.*/
    pub fn list_payments_for_invoice(
        &self,
        invoice_id: &str,
    ) -> request::ListPaymentsForInvoiceRequest {
        request::ListPaymentsForInvoiceRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create an invoice payment

Creates a new invoice payment object. Returns an invoice payment object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-payments/#create-an-invoice-payment>.*/
    pub fn create_invoice_payment(
        &self,
        invoice_id: &str,
    ) -> request::CreateInvoicePaymentRequest {
        request::CreateInvoicePaymentRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            amount: None,
            paid_at: None,
            paid_date: None,
            notes: None,
        }
    }
    /**Delete an invoice payment

Delete an invoice payment. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-payments/#delete-an-invoice-payment>.*/
    pub fn delete_invoice_payment(
        &self,
        invoice_id: &str,
        payment_id: &str,
    ) -> request::DeleteInvoicePaymentRequest {
        request::DeleteInvoicePaymentRequest {
            client: &self,
            invoice_id: invoice_id.to_owned(),
            payment_id: payment_id.to_owned(),
        }
    }
    /**List all projects

Returns a list of your projects. The projects are returned sorted by creation date, with the most recently created projects appearing first.

The response contains an object with a projects property that contains an array of up to per_page projects. Each entry in the array is a separate project object. If no more projects are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your projects.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#list-all-projects>.*/
    pub fn list_projects(&self) -> request::ListProjectsRequest {
        request::ListProjectsRequest {
            client: &self,
            is_active: None,
            client_id: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a project

Creates a new project object. Returns a project object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#create-a-project>.*/
    pub fn create_project(&self) -> request::CreateProjectRequest {
        request::CreateProjectRequest {
            client: &self,
            client_id: None,
            name: None,
            code: None,
            is_active: None,
            is_billable: None,
            is_fixed_fee: None,
            bill_by: None,
            hourly_rate: None,
            budget: None,
            budget_by: None,
            budget_is_monthly: None,
            notify_when_over_budget: None,
            over_budget_notification_percentage: None,
            show_budget_to_all: None,
            cost_budget: None,
            cost_budget_include_expenses: None,
            fee: None,
            notes: None,
            starts_on: None,
            ends_on: None,
        }
    }
    /**Retrieve a project

Retrieves the project with the given ID. Returns a project object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#retrieve-a-project>.*/
    pub fn retrieve_project(&self, project_id: &str) -> request::RetrieveProjectRequest {
        request::RetrieveProjectRequest {
            client: &self,
            project_id: project_id.to_owned(),
        }
    }
    /**Delete a project

Deletes a project and any time entries or expenses tracked to it.
However, invoices associated with the project will not be deleted.
If you don’t want the project’s time entries and expenses to be deleted, you should archive the project instead.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#delete-a-project>.*/
    pub fn delete_project(&self, project_id: &str) -> request::DeleteProjectRequest {
        request::DeleteProjectRequest {
            client: &self,
            project_id: project_id.to_owned(),
        }
    }
    /**Update a project

Updates the specific project by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a project object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#update-a-project>.*/
    pub fn update_project(&self, project_id: &str) -> request::UpdateProjectRequest {
        request::UpdateProjectRequest {
            client: &self,
            project_id: project_id.to_owned(),
            client_id: None,
            name: None,
            code: None,
            is_active: None,
            is_billable: None,
            is_fixed_fee: None,
            bill_by: None,
            hourly_rate: None,
            budget: None,
            budget_by: None,
            budget_is_monthly: None,
            notify_when_over_budget: None,
            over_budget_notification_percentage: None,
            show_budget_to_all: None,
            cost_budget: None,
            cost_budget_include_expenses: None,
            fee: None,
            notes: None,
            starts_on: None,
            ends_on: None,
        }
    }
    /**List all task assignments for a specific project

Returns a list of your task assignments for the project identified by PROJECT_ID. The task assignments are returned sorted by creation date, with the most recently created task assignments appearing first.

The response contains an object with a task_assignments property that contains an array of up to per_page task assignments. Each entry in the array is a separate task assignment object. If no more task assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your task assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#list-all-task-assignments-for-a-specific-project>.*/
    pub fn list_task_assignments_for_specific_project(
        &self,
        project_id: &str,
    ) -> request::ListTaskAssignmentsForSpecificProjectRequest {
        request::ListTaskAssignmentsForSpecificProjectRequest {
            client: &self,
            project_id: project_id.to_owned(),
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a task assignment

Creates a new task assignment object. Returns a task assignment object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#create-a-task-assignment>.*/
    pub fn create_task_assignment(
        &self,
        project_id: &str,
    ) -> request::CreateTaskAssignmentRequest {
        request::CreateTaskAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            task_id: None,
            is_active: None,
            billable: None,
            hourly_rate: None,
            budget: None,
        }
    }
    /**Retrieve a task assignment

Retrieves the task assignment with the given ID. Returns a task assignment object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#retrieve-a-task-assignment>.*/
    pub fn retrieve_task_assignment(
        &self,
        project_id: &str,
        task_assignment_id: &str,
    ) -> request::RetrieveTaskAssignmentRequest {
        request::RetrieveTaskAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            task_assignment_id: task_assignment_id.to_owned(),
        }
    }
    /**Delete a task assignment

Delete a task assignment. Deleting a task assignment is only possible if it has no time entries associated with it. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#delete-a-task-assignment>.*/
    pub fn delete_task_assignment(
        &self,
        project_id: &str,
        task_assignment_id: &str,
    ) -> request::DeleteTaskAssignmentRequest {
        request::DeleteTaskAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            task_assignment_id: task_assignment_id.to_owned(),
        }
    }
    /**Update a task assignment

Updates the specific task assignment by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a task assignment object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#update-a-task-assignment>.*/
    pub fn update_task_assignment(
        &self,
        project_id: &str,
        task_assignment_id: &str,
    ) -> request::UpdateTaskAssignmentRequest {
        request::UpdateTaskAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            task_assignment_id: task_assignment_id.to_owned(),
            is_active: None,
            billable: None,
            hourly_rate: None,
            budget: None,
        }
    }
    /**List all user assignments for a specific project

Returns a list of user assignments for the project identified by PROJECT_ID. The user assignments are returned sorted by creation date, with the most recently created user assignments appearing first.

The response contains an object with a user_assignments property that contains an array of up to per_page user assignments. Each entry in the array is a separate user assignment object. If no more user assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your user assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#list-all-user-assignments-for-a-specific-project>.*/
    pub fn list_user_assignments_for_specific_project(
        &self,
        project_id: &str,
    ) -> request::ListUserAssignmentsForSpecificProjectRequest {
        request::ListUserAssignmentsForSpecificProjectRequest {
            client: &self,
            project_id: project_id.to_owned(),
            user_id: None,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a user assignment

Creates a new user assignment object. Returns a user assignment object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#create-a-user-assignment>.*/
    pub fn create_user_assignment(
        &self,
        project_id: &str,
    ) -> request::CreateUserAssignmentRequest {
        request::CreateUserAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            user_id: None,
            is_active: None,
            is_project_manager: None,
            use_default_rates: None,
            hourly_rate: None,
            budget: None,
        }
    }
    /**Retrieve a user assignment

Retrieves the user assignment with the given ID. Returns a user assignment object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#retrieve-a-user-assignment>.*/
    pub fn retrieve_user_assignment(
        &self,
        project_id: &str,
        user_assignment_id: &str,
    ) -> request::RetrieveUserAssignmentRequest {
        request::RetrieveUserAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            user_assignment_id: user_assignment_id.to_owned(),
        }
    }
    /**Delete a user assignment

Delete a user assignment. Deleting a user assignment is only possible if it has no time entries or expenses associated with it. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#delete-a-user-assignment>.*/
    pub fn delete_user_assignment(
        &self,
        project_id: &str,
        user_assignment_id: &str,
    ) -> request::DeleteUserAssignmentRequest {
        request::DeleteUserAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            user_assignment_id: user_assignment_id.to_owned(),
        }
    }
    /**Update a user assignment

Updates the specific user assignment by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a user assignment object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#update-a-user-assignment>.*/
    pub fn update_user_assignment(
        &self,
        project_id: &str,
        user_assignment_id: &str,
    ) -> request::UpdateUserAssignmentRequest {
        request::UpdateUserAssignmentRequest {
            client: &self,
            project_id: project_id.to_owned(),
            user_assignment_id: user_assignment_id.to_owned(),
            is_active: None,
            is_project_manager: None,
            use_default_rates: None,
            hourly_rate: None,
            budget: None,
        }
    }
    /**Expense Categories Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#expense-categories-report>.*/
    pub fn expense_categories_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::ExpenseCategoriesReportRequest {
        request::ExpenseCategoriesReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Clients Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#clients-report>.*/
    pub fn clients_expenses_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::ClientsExpensesReportRequest {
        request::ClientsExpensesReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Projects Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#projects-report>.*/
    pub fn projects_expenses_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::ProjectsExpensesReportRequest {
        request::ProjectsExpensesReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Team Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#team-report>.*/
    pub fn team_expenses_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::TeamExpensesReportRequest {
        request::TeamExpensesReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Project Budget Report

The response contains an object with a results property that contains an array of up to per_page results. Each entry in the array is a separate result object. If no more results are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your results.

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/project-budget-report/#project-budget-report>.*/
    pub fn project_budget_report(&self) -> request::ProjectBudgetReportRequest {
        request::ProjectBudgetReportRequest {
            client: &self,
            page: None,
            per_page: None,
            is_active: None,
        }
    }
    /**Clients Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#clients-report>.*/
    pub fn clients_time_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::ClientsTimeReportRequest {
        request::ClientsTimeReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Projects Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#projects-report>.*/
    pub fn projects_time_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::ProjectsTimeReportRequest {
        request::ProjectsTimeReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Tasks Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#tasks-report>.*/
    pub fn tasks_report(&self, from: &str, to: &str) -> request::TasksReportRequest {
        request::TasksReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Team Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#team-report>.*/
    pub fn team_time_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::TeamTimeReportRequest {
        request::TeamTimeReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Uninvoiced Report

The response contains an object with a results property that contains an array of up to per_page results. Each entry in the array is a separate result object. If no more results are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your results.

Note: Each request requires both the from and to parameters to be supplied in the URL’s query string. The timeframe supplied cannot exceed 1 year (365 days).

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/uninvoiced-report/#uninvoiced-report>.*/
    pub fn uninvoiced_report(
        &self,
        from: &str,
        to: &str,
    ) -> request::UninvoicedReportRequest {
        request::UninvoicedReportRequest {
            client: &self,
            from: from.to_owned(),
            to: to.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**List all roles

Returns a list of roles in the account. The roles are returned sorted by creation date, with the most recently created roles appearing first.

The response contains an object with a roles property that contains an array of up to per_page roles. Each entry in the array is a separate role object. If no more roles are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your roles.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#list-all-roles>.*/
    pub fn list_roles(&self) -> request::ListRolesRequest {
        request::ListRolesRequest {
            client: &self,
            page: None,
            per_page: None,
        }
    }
    /**Create a role

Creates a new role object. Returns a role object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#create-a-role>.*/
    pub fn create_role(&self) -> request::CreateRoleRequest {
        request::CreateRoleRequest {
            client: &self,
            name: None,
            user_ids: None,
        }
    }
    /**Retrieve a role

Retrieves the role with the given ID. Returns a role object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#retrieve-a-role>.*/
    pub fn retrieve_role(&self, role_id: &str) -> request::RetrieveRoleRequest {
        request::RetrieveRoleRequest {
            client: &self,
            role_id: role_id.to_owned(),
        }
    }
    /**Delete a role

Delete a role. Deleting a role will unlink it from any users it was assigned to. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#delete-a-role>.*/
    pub fn delete_role(&self, role_id: &str) -> request::DeleteRoleRequest {
        request::DeleteRoleRequest {
            client: &self,
            role_id: role_id.to_owned(),
        }
    }
    /**Update a role

Updates the specific role by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a role object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#update-a-role>.*/
    pub fn update_role(&self, role_id: &str) -> request::UpdateRoleRequest {
        request::UpdateRoleRequest {
            client: &self,
            role_id: role_id.to_owned(),
            name: None,
            user_ids: None,
        }
    }
    /**List all task assignments

Returns a list of your task assignments. The task assignments are returned sorted by creation date, with the most recently created task assignments appearing first.

The response contains an object with a task_assignments property that contains an array of up to per_page task assignments. Each entry in the array is a separate task assignment object. If no more task assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your task assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#list-all-task-assignments>.*/
    pub fn list_task_assignments(&self) -> request::ListTaskAssignmentsRequest {
        request::ListTaskAssignmentsRequest {
            client: &self,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**List all tasks

Returns a list of your tasks. The tasks are returned sorted by creation date, with the most recently created tasks appearing first.

The response contains an object with a tasks property that contains an array of up to per_page tasks. Each entry in the array is a separate task object. If no more tasks are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your tasks.

See endpoint docs at <https://help.getharvest.com/api-v2/tasks-api/tasks/tasks/#list-all-tasks>.*/
    pub fn list_tasks(&self) -> request::ListTasksRequest {
        request::ListTasksRequest {
            client: &self,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a task

Creates a new task object. Returns a task object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/tasks-api/tasks/tasks/#create-a-task>.*/
    pub fn create_task(&self) -> request::CreateTaskRequest {
        request::CreateTaskRequest {
            client: &self,
            name: None,
            billable_by_default: None,
            default_hourly_rate: None,
            is_default: None,
            is_active: None,
        }
    }
    /**Retrieve a task

Retrieves the task with the given ID. Returns a task object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/tasks-api/tasks/tasks/#retrieve-a-task>.*/
    pub fn retrieve_task(&self, task_id: &str) -> request::RetrieveTaskRequest {
        request::RetrieveTaskRequest {
            client: &self,
            task_id: task_id.to_owned(),
        }
    }
    /**Delete a task

Delete a task. Deleting a task is only possible if it has no time entries associated with it. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/tasks-api/tasks/tasks/#delete-a-task>.*/
    pub fn delete_task(&self, task_id: &str) -> request::DeleteTaskRequest {
        request::DeleteTaskRequest {
            client: &self,
            task_id: task_id.to_owned(),
        }
    }
    /**Update a task

Updates the specific task by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a task object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/tasks-api/tasks/tasks/#update-a-task>.*/
    pub fn update_task(&self, task_id: &str) -> request::UpdateTaskRequest {
        request::UpdateTaskRequest {
            client: &self,
            task_id: task_id.to_owned(),
            name: None,
            billable_by_default: None,
            default_hourly_rate: None,
            is_default: None,
            is_active: None,
        }
    }
    /**List all time entries

Returns a list of time entries. The time entries are returned sorted by spent_date date. At this time, the sort option can’t be customized.

The response contains an object with a time_entries property that contains an array of up to per_page time entries. Each entry in the array is a separate time entry object. If no more time entries are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your time entries.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#list-all-time-entries>.*/
    pub fn list_time_entries(&self) -> request::ListTimeEntriesRequest {
        request::ListTimeEntriesRequest {
            client: &self,
            user_id: None,
            client_id: None,
            project_id: None,
            task_id: None,
            external_reference_id: None,
            is_billed: None,
            is_running: None,
            updated_since: None,
            from: None,
            to: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a time entry

Creates a new time entry object. Returns a time entry object and a 201 Created response code if the call succeeded.

You should only use this method to create time entries when your account is configured to track time via duration. You can verify this by visiting the Settings page in your Harvest account or by checking if wants_timestamp_timers is false in the Company API.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#create-a-time-entry-via-duration>.*/
    pub fn create_time_entry(&self) -> request::CreateTimeEntryRequest {
        request::CreateTimeEntryRequest {
            client: &self,
            user_id: None,
            project_id: None,
            task_id: None,
            spent_date: None,
            started_time: None,
            ended_time: None,
            notes: None,
            external_reference: None,
            hours: None,
        }
    }
    /**Retrieve a time entry

Retrieves the time entry with the given ID. Returns a time entry object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#retrieve-a-time-entry>.*/
    pub fn retrieve_time_entry(
        &self,
        time_entry_id: &str,
    ) -> request::RetrieveTimeEntryRequest {
        request::RetrieveTimeEntryRequest {
            client: &self,
            time_entry_id: time_entry_id.to_owned(),
        }
    }
    /**Delete a time entry

Delete a time entry. Deleting a time entry is only possible if it’s not closed and the associated project and task haven’t been archived.  However, Admins can delete closed entries. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#delete-a-time-entry>.*/
    pub fn delete_time_entry(
        &self,
        time_entry_id: &str,
    ) -> request::DeleteTimeEntryRequest {
        request::DeleteTimeEntryRequest {
            client: &self,
            time_entry_id: time_entry_id.to_owned(),
        }
    }
    /**Update a time entry

Updates the specific time entry by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a time entry object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#update-a-time-entry>.*/
    pub fn update_time_entry(
        &self,
        time_entry_id: &str,
    ) -> request::UpdateTimeEntryRequest {
        request::UpdateTimeEntryRequest {
            client: &self,
            time_entry_id: time_entry_id.to_owned(),
            project_id: None,
            task_id: None,
            spent_date: None,
            started_time: None,
            ended_time: None,
            hours: None,
            notes: None,
            external_reference: None,
        }
    }
    /**Delete a time entry’s external reference

Delete a time entry’s external reference. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#delete-a-time-entrys-external-reference>.*/
    pub fn delete_time_entry_external_reference(
        &self,
        time_entry_id: &str,
    ) -> request::DeleteTimeEntryExternalReferenceRequest {
        request::DeleteTimeEntryExternalReferenceRequest {
            client: &self,
            time_entry_id: time_entry_id.to_owned(),
        }
    }
    /**Restart a stopped time entry

Restarting a time entry is only possible if it isn’t currently running. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#restart-a-stopped-time-entry>.*/
    pub fn restart_stopped_time_entry(
        &self,
        time_entry_id: &str,
    ) -> request::RestartStoppedTimeEntryRequest {
        request::RestartStoppedTimeEntryRequest {
            client: &self,
            time_entry_id: time_entry_id.to_owned(),
        }
    }
    /**Stop a running time entry

Stopping a time entry is only possible if it’s currently running. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#stop-a-running-time-entry>.*/
    pub fn stop_running_time_entry(
        &self,
        time_entry_id: &str,
    ) -> request::StopRunningTimeEntryRequest {
        request::StopRunningTimeEntryRequest {
            client: &self,
            time_entry_id: time_entry_id.to_owned(),
        }
    }
    /**List all user assignments

Returns a list of your projects user assignments, active and archived. The user assignments are returned sorted by creation date, with the most recently created user assignments appearing first.

The response contains an object with a user_assignments property that contains an array of up to per_page user assignments. Each entry in the array is a separate user assignment object. If no more user assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your user assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#list-all-user-assignments>.*/
    pub fn list_user_assignments(&self) -> request::ListUserAssignmentsRequest {
        request::ListUserAssignmentsRequest {
            client: &self,
            user_id: None,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**List all users

Returns a list of your users. The users are returned sorted by creation date, with the most recently created users appearing first.

The response contains an object with a users property that contains an array of up to per_page users. Each entry in the array is a separate user object. If no more users are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your users.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#list-all-users>.*/
    pub fn list_users(&self) -> request::ListUsersRequest {
        request::ListUsersRequest {
            client: &self,
            is_active: None,
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
    /**Create a user

Creates a new user object and sends an invitation email to the address specified in the email parameter. Returns a user object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#create-a-user>.*/
    pub fn create_user(&self) -> request::CreateUserRequest {
        request::CreateUserRequest {
            client: &self,
            first_name: None,
            last_name: None,
            email: None,
            timezone: None,
            has_access_to_all_future_projects: None,
            is_contractor: None,
            is_active: None,
            weekly_capacity: None,
            default_hourly_rate: None,
            cost_rate: None,
            roles: None,
        }
    }
    /**Retrieve the currently authenticated user

Retrieves the currently authenticated user. Returns a user object and a 200 OK response code.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#retrieve-the-currently-authenticated-user>.*/
    pub fn retrieve_the_currently_authenticated_user(
        &self,
    ) -> request::RetrieveTheCurrentlyAuthenticatedUserRequest {
        request::RetrieveTheCurrentlyAuthenticatedUserRequest {
            client: &self,
        }
    }
    /**List active project assignments for the currently authenticated user

Returns a list of your active project assignments for the currently authenticated user. The project assignments are returned sorted by creation date, with the most recently created project assignments appearing first.

The response contains an object with a project_assignments property that contains an array of up to per_page project assignments. Each entry in the array is a separate project assignment object. If no more project assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your project assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/project-assignments/#list-active-project-assignments-for-the-currently-authenticated-user>.*/
    pub fn list_active_project_assignments_for_the_currently_authenticated_user(
        &self,
    ) -> request::ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest {
        request::ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest {
            client: &self,
            page: None,
            per_page: None,
        }
    }
    /**Retrieve a user

Retrieves the user with the given ID. Returns a user object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#retrieve-a-user>.*/
    pub fn retrieve_user(&self, user_id: &str) -> request::RetrieveUserRequest {
        request::RetrieveUserRequest {
            client: &self,
            user_id: user_id.to_owned(),
        }
    }
    /**Delete a user

Delete a user. Deleting a user is only possible if they have no time entries or expenses associated with them. Returns a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#delete-a-user>.*/
    pub fn delete_user(&self, user_id: &str) -> request::DeleteUserRequest {
        request::DeleteUserRequest {
            client: &self,
            user_id: user_id.to_owned(),
        }
    }
    /**Update a user

Updates the specific user by setting the values of the parameters passed. Any parameters not provided will be left unchanged. Returns a user object and a 200 OK response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#update-a-user>.*/
    pub fn update_user(&self, user_id: &str) -> request::UpdateUserRequest {
        request::UpdateUserRequest {
            client: &self,
            user_id: user_id.to_owned(),
            first_name: None,
            last_name: None,
            email: None,
            timezone: None,
            has_access_to_all_future_projects: None,
            is_contractor: None,
            is_active: None,
            weekly_capacity: None,
            default_hourly_rate: None,
            cost_rate: None,
            roles: None,
        }
    }
    /**List all billable rates for a specific user

Returns a list of billable rates for the user identified by USER_ID. The billable rates are returned sorted by start_date, with the oldest starting billable rates appearing first.

The response contains an object with a billable_rates property that contains an array of up to per_page billable rates. Each entry in the array is a separate billable rate object. If no more billable rates are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your billable rates.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/billable-rates/#list-all-billable-rates-for-a-specific-user>.*/
    pub fn list_billable_rates_for_specific_user(
        &self,
        user_id: &str,
    ) -> request::ListBillableRatesForSpecificUserRequest {
        request::ListBillableRatesForSpecificUserRequest {
            client: &self,
            user_id: user_id.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Create a billable rate

Creates a new billable rate object. Returns a billable rate object and a 201 Created response code if the call succeeded.


  Creating a billable rate with no start_date will replace a user’s existing rate(s).
  Creating a billable rate with a start_date that is before a user’s existing rate(s) will replace those billable rates with the new one.


See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/billable-rates/#create-a-billable-rate>.*/
    pub fn create_billable_rate(
        &self,
        user_id: &str,
    ) -> request::CreateBillableRateRequest {
        request::CreateBillableRateRequest {
            client: &self,
            user_id: user_id.to_owned(),
            amount: None,
            start_date: None,
        }
    }
    /**Retrieve a billable rate

Retrieves the billable rate with the given ID. Returns a billable rate object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/billable-rates/#retrieve-a-billable-rate>.*/
    pub fn retrieve_billable_rate(
        &self,
        user_id: &str,
        billable_rate_id: &str,
    ) -> request::RetrieveBillableRateRequest {
        request::RetrieveBillableRateRequest {
            client: &self,
            user_id: user_id.to_owned(),
            billable_rate_id: billable_rate_id.to_owned(),
        }
    }
    /**List all cost rates for a specific user

Returns a list of cost rates for the user identified by USER_ID. The cost rates are returned sorted by start_date, with the oldest starting cost rates appearing first.

The response contains an object with a cost_rates property that contains an array of up to per_page cost rates. Each entry in the array is a separate cost rate object. If no more cost rates are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your cost rates.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/cost-rates/#list-all-cost-rates-for-a-specific-user>.*/
    pub fn list_cost_rates_for_specific_user(
        &self,
        user_id: &str,
    ) -> request::ListCostRatesForSpecificUserRequest {
        request::ListCostRatesForSpecificUserRequest {
            client: &self,
            user_id: user_id.to_owned(),
            page: None,
            per_page: None,
        }
    }
    /**Create a cost rate

Creates a new cost rate object. Returns a cost rate object and a 201 Created response code if the call succeeded.


  Creating a cost rate with no start_date will replace a user’s existing rate(s).
  Creating a cost rate with a start_date that is before a user’s existing rate(s) will replace those cost rates with the new one.


See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/cost-rates/#create-a-cost-rate>.*/
    pub fn create_cost_rate(&self, user_id: &str) -> request::CreateCostRateRequest {
        request::CreateCostRateRequest {
            client: &self,
            user_id: user_id.to_owned(),
            amount: None,
            start_date: None,
        }
    }
    /**Retrieve a cost rate

Retrieves the cost rate with the given ID. Returns a cost rate object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/cost-rates/#retrieve-a-cost-rate>.*/
    pub fn retrieve_cost_rate(
        &self,
        user_id: &str,
        cost_rate_id: &str,
    ) -> request::RetrieveCostRateRequest {
        request::RetrieveCostRateRequest {
            client: &self,
            user_id: user_id.to_owned(),
            cost_rate_id: cost_rate_id.to_owned(),
        }
    }
    /**List active project assignments

Returns a list of active project assignments for the user identified by USER_ID. The project assignments are returned sorted by creation date, with the most recently created project assignments appearing first.

The response contains an object with a project_assignments property that contains an array of up to per_page project assignments. Each entry in the array is a separate project assignment object. If no more project assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your project assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/project-assignments/#list-active-project-assignments>.*/
    pub fn list_active_project_assignments(
        &self,
        user_id: &str,
    ) -> request::ListActiveProjectAssignmentsRequest {
        request::ListActiveProjectAssignmentsRequest {
            client: &self,
            user_id: user_id.to_owned(),
            updated_since: None,
            page: None,
            per_page: None,
        }
    }
}
pub enum HarvestAuthentication {
    BearerAuth { bearer_auth: String, account_auth: String },
}
impl HarvestAuthentication {
    pub fn from_env() -> Self {
        Self::BearerAuth {
            bearer_auth: std::env::var("HARVEST_BEARER_AUTH")
                .expect("Environment variable HARVEST_BEARER_AUTH is not set."),
            account_auth: std::env::var("HARVEST_ACCOUNT_AUTH")
                .expect("Environment variable HARVEST_ACCOUNT_AUTH is not set."),
        }
    }
}
