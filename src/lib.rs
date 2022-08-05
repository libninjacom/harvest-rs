//! [`HarvestClient`](struct.HarvestClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
pub mod model;
pub mod request_model;
use crate::model::*;

pub struct HarvestClient {
    pub(crate) client: httpclient::Client,
    authentication: Option<HarvestAuthentication>,
}
impl HarvestClient {}
impl HarvestClient {
    pub fn new(url: &str) -> Self {
        let client = httpclient::Client::new(Some(url.to_string()));
        let authentication = None;
        Self { client, authentication }
    }
    pub fn with_authentication(mut self, authentication: HarvestAuthentication) -> Self {
        self.authentication = Some(authentication);
        self
    }
    pub fn authenticate<'a>(
        &self,
        mut r: httpclient::RequestBuilder<'a>,
    ) -> httpclient::RequestBuilder<'a> {
        if let Some(ref authentication) = self.authentication {
            match authentication {
                HarvestAuthentication::BearerAuth { bearer_auth, account_auth } => {
                    r = r.bearer_auth(bearer_auth);
                    r = r.header("Harvest-Account-Id", account_auth);
                }
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
    pub fn list_clients(&self) -> request_model::ListClientsRequest {
        request_model::ListClientsRequest {
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
    pub fn create_client(
        &self,
        name: String,
        is_active: bool,
        address: String,
        currency: String,
    ) -> request_model::CreateClientRequest {
        request_model::CreateClientRequest {
            client: &self,
            name,
            is_active,
            address,
            currency,
        }
    }
    /**Retrieve a client

Retrieves the client with the given ID. Returns a client object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/clients/#retrieve-a-client>.*/
    pub fn retrieve_client(
        &self,
        client_id: String,
    ) -> request_model::RetrieveClientRequest {
        request_model::RetrieveClientRequest {
            client: &self,
            client_id,
        }
    }
    /**Retrieve a company

Retrieves the company for the currently authenticated user. Returns a
company object and a 200 OK response code.

See endpoint docs at <https://help.getharvest.com/api-v2/company-api/company/company/#retrieve-a-company>.*/
    pub fn retrieve_company(&self) -> request_model::RetrieveCompanyRequest {
        request_model::RetrieveCompanyRequest {
            client: &self,
        }
    }
    /**List all contacts

Returns a list of your contacts. The contacts are returned sorted by creation date, with the most recently created contacts appearing first.

The response contains an object with a contacts property that contains an array of up to per_page contacts. Each entry in the array is a separate contact object. If no more contacts are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your contacts.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#list-all-contacts>.*/
    pub fn list_contacts(&self) -> request_model::ListContactsRequest {
        request_model::ListContactsRequest {
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
    pub fn create_contact(
        &self,
        client_id: i64,
        title: String,
        first_name: String,
        last_name: String,
        email: String,
        phone_office: String,
        phone_mobile: String,
        fax: String,
    ) -> request_model::CreateContactRequest {
        request_model::CreateContactRequest {
            client: &self,
            client_id,
            title,
            first_name,
            last_name,
            email,
            phone_office,
            phone_mobile,
            fax,
        }
    }
    /**Retrieve a contact

Retrieves the contact with the given ID. Returns a contact object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/clients-api/clients/contacts/#retrieve-a-contact>.*/
    pub fn retrieve_contact(
        &self,
        contact_id: String,
    ) -> request_model::RetrieveContactRequest {
        request_model::RetrieveContactRequest {
            client: &self,
            contact_id,
        }
    }
    /**List all estimate item categories

Returns a list of your estimate item categories. The estimate item categories are returned sorted by creation date, with the most recently created estimate item categories appearing first.

The response contains an object with a estimate_item_categories property that contains an array of up to per_page estimate item categories. Each entry in the array is a separate estimate item category object. If no more estimate item categories are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your estimate item categories.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#list-all-estimate-item-categories>.*/
    pub fn list_estimate_item_categories(
        &self,
    ) -> request_model::ListEstimateItemCategoriesRequest {
        request_model::ListEstimateItemCategoriesRequest {
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
        name: String,
    ) -> request_model::CreateEstimateItemCategoryRequest {
        request_model::CreateEstimateItemCategoryRequest {
            client: &self,
            name,
        }
    }
    /**Retrieve an estimate item category

Retrieves the estimate item category with the given ID. Returns an estimate item category object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-item-categories/#retrieve-an-estimate-item-category>.*/
    pub fn retrieve_estimate_item_category(
        &self,
        estimate_item_category_id: String,
    ) -> request_model::RetrieveEstimateItemCategoryRequest {
        request_model::RetrieveEstimateItemCategoryRequest {
            client: &self,
            estimate_item_category_id,
        }
    }
    /**List all estimates

Returns a list of your estimates. The estimates are returned sorted by issue date, with the most recently issued estimates appearing first.

The response contains an object with a estimates property that contains an array of up to per_page estimates. Each entry in the array is a separate estimate object. If no more estimates are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your estimates.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#list-all-estimates>.*/
    pub fn list_estimates(&self) -> request_model::ListEstimatesRequest {
        request_model::ListEstimatesRequest {
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
    pub fn create_estimate(
        &self,
        client_id: i64,
        number: String,
        purchase_order: String,
        tax: f64,
        tax_2: f64,
        discount: f64,
        subject: String,
        notes: String,
        currency: String,
        issue_date: String,
        line_items: Vec<serde_json::Value>,
    ) -> request_model::CreateEstimateRequest {
        request_model::CreateEstimateRequest {
            client: &self,
            client_id,
            number,
            purchase_order,
            tax,
            tax_2,
            discount,
            subject,
            notes,
            currency,
            issue_date,
            line_items,
        }
    }
    /**Retrieve an estimate

Retrieves the estimate with the given ID. Returns an estimate object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimates/#retrieve-an-estimate>.*/
    pub fn retrieve_estimate(
        &self,
        estimate_id: String,
    ) -> request_model::RetrieveEstimateRequest {
        request_model::RetrieveEstimateRequest {
            client: &self,
            estimate_id,
        }
    }
    /**List all messages for an estimate

Returns a list of messages associated with a given estimate. The estimate messages are returned sorted by creation date, with the most recently created messages appearing first.

The response contains an object with an estimate_messages property that contains an array of up to per_page messages. Each entry in the array is a separate message object. If no more messages are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your messages.

See endpoint docs at <https://help.getharvest.com/api-v2/estimates-api/estimates/estimate-messages/#list-all-messages-for-an-estimate>.*/
    pub fn list_messages_for_estimate(
        &self,
        estimate_id: String,
    ) -> request_model::ListMessagesForEstimateRequest {
        request_model::ListMessagesForEstimateRequest {
            client: &self,
            estimate_id,
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
        estimate_id: String,
        event_type: String,
        recipients: Vec<serde_json::Value>,
        subject: String,
        body: String,
        send_me_a_copy: bool,
    ) -> request_model::CreateEstimateMessageRequest {
        request_model::CreateEstimateMessageRequest {
            client: &self,
            estimate_id,
            event_type,
            recipients,
            subject,
            body,
            send_me_a_copy,
        }
    }
    /**List all expense categories

Returns a list of your expense categories. The expense categories are returned sorted by creation date, with the most recently created expense categories appearing first.

The response contains an object with a expense_categories property that contains an array of up to per_page expense categories. Each entry in the array is a separate expense category object. If no more expense categories are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your expense categories.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#list-all-expense-categories>.*/
    pub fn list_expense_categories(
        &self,
    ) -> request_model::ListExpenseCategoriesRequest {
        request_model::ListExpenseCategoriesRequest {
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
    pub fn create_expense_category(
        &self,
        name: String,
        unit_name: String,
        unit_price: f64,
        is_active: bool,
    ) -> request_model::CreateExpenseCategoryRequest {
        request_model::CreateExpenseCategoryRequest {
            client: &self,
            name,
            unit_name,
            unit_price,
            is_active,
        }
    }
    /**Retrieve an expense category

Retrieves the expense category with the given ID. Returns an expense category object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expense-categories/#retrieve-an-expense-category>.*/
    pub fn retrieve_expense_category(
        &self,
        expense_category_id: String,
    ) -> request_model::RetrieveExpenseCategoryRequest {
        request_model::RetrieveExpenseCategoryRequest {
            client: &self,
            expense_category_id,
        }
    }
    /**List all expenses

Returns a list of your expenses. If accessing this endpoint as an Administrator, all expenses in the account will be returned. If accessing this endpoint as a Manager, all expenses for assigned teammates and managed projects will be returned. The expenses are returned sorted by the spent_at date, with the most recent expenses appearing first.

The response contains an object with a expenses property that contains an array of up to per_page expenses. Each entry in the array is a separate expense object. If no more expenses are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your expenses.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#list-all-expenses>.*/
    pub fn list_expenses(&self) -> request_model::ListExpensesRequest {
        request_model::ListExpensesRequest {
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
    pub fn create_expense(
        &self,
        user_id: i64,
        project_id: i64,
        expense_category_id: i64,
        spent_date: String,
        units: i64,
        total_cost: f64,
        notes: String,
        billable: bool,
        receipt: String,
    ) -> request_model::CreateExpenseRequest {
        request_model::CreateExpenseRequest {
            client: &self,
            user_id,
            project_id,
            expense_category_id,
            spent_date,
            units,
            total_cost,
            notes,
            billable,
            receipt,
        }
    }
    /**Retrieve an expense

Retrieves the expense with the given ID. Returns an expense object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/expenses-api/expenses/expenses/#retrieve-an-expense>.*/
    pub fn retrieve_expense(
        &self,
        expense_id: String,
    ) -> request_model::RetrieveExpenseRequest {
        request_model::RetrieveExpenseRequest {
            client: &self,
            expense_id,
        }
    }
    /**List all invoice item categories

Returns a list of your invoice item categories. The invoice item categories are returned sorted by creation date, with the most recently created invoice item categories appearing first.

The response contains an object with a invoice_item_categories property that contains an array of up to per_page invoice item categories. Each entry in the array is a separate invoice item category object. If no more invoice item categories are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your invoice item categories.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#list-all-invoice-item-categories>.*/
    pub fn list_invoice_item_categories(
        &self,
    ) -> request_model::ListInvoiceItemCategoriesRequest {
        request_model::ListInvoiceItemCategoriesRequest {
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
        name: String,
    ) -> request_model::CreateInvoiceItemCategoryRequest {
        request_model::CreateInvoiceItemCategoryRequest {
            client: &self,
            name,
        }
    }
    /**Retrieve an invoice item category

Retrieves the invoice item category with the given ID. Returns an invoice item category object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-item-categories/#retrieve-an-invoice-item-category>.*/
    pub fn retrieve_invoice_item_category(
        &self,
        invoice_item_category_id: String,
    ) -> request_model::RetrieveInvoiceItemCategoryRequest {
        request_model::RetrieveInvoiceItemCategoryRequest {
            client: &self,
            invoice_item_category_id,
        }
    }
    /**List all invoices

Returns a list of your invoices. The invoices are returned sorted by issue date, with the most recently issued invoices appearing first.

The response contains an object with a invoices property that contains an array of up to per_page invoices. Each entry in the array is a separate invoice object. If no more invoices are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your invoices.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#list-all-invoices>.*/
    pub fn list_invoices(&self) -> request_model::ListInvoicesRequest {
        request_model::ListInvoicesRequest {
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
    pub fn create_invoice(
        &self,
        client_id: i64,
        retainer_id: i64,
        estimate_id: i64,
        number: String,
        purchase_order: String,
        tax: f64,
        tax_2: f64,
        discount: f64,
        subject: String,
        notes: String,
        currency: String,
        issue_date: String,
        due_date: String,
        payment_term: String,
        line_items: Vec<serde_json::Value>,
    ) -> request_model::CreateInvoiceRequest {
        request_model::CreateInvoiceRequest {
            client: &self,
            client_id,
            retainer_id,
            estimate_id,
            number,
            purchase_order,
            tax,
            tax_2,
            discount,
            subject,
            notes,
            currency,
            issue_date,
            due_date,
            payment_term,
            line_items_import: None,
            line_items,
        }
    }
    /**Retrieve an invoice

Retrieves the invoice with the given ID. Returns an invoice object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoices/#retrieve-an-invoice>.*/
    pub fn retrieve_invoice(
        &self,
        invoice_id: String,
    ) -> request_model::RetrieveInvoiceRequest {
        request_model::RetrieveInvoiceRequest {
            client: &self,
            invoice_id,
        }
    }
    /**List all messages for an invoice

Returns a list of messages associated with a given invoice. The invoice messages are returned sorted by creation date, with the most recently created messages appearing first.

The response contains an object with an invoice_messages property that contains an array of up to per_page messages. Each entry in the array is a separate message object. If no more messages are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your messages.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-messages/#list-all-messages-for-an-invoice>.*/
    pub fn list_messages_for_invoice(
        &self,
        invoice_id: String,
    ) -> request_model::ListMessagesForInvoiceRequest {
        request_model::ListMessagesForInvoiceRequest {
            client: &self,
            invoice_id,
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
        invoice_id: String,
        event_type: String,
        recipients: Vec<serde_json::Value>,
        subject: String,
        body: String,
        include_link_to_client_invoice: bool,
        attach_pdf: bool,
        send_me_a_copy: bool,
        thank_you: bool,
    ) -> request_model::CreateInvoiceMessageRequest {
        request_model::CreateInvoiceMessageRequest {
            client: &self,
            invoice_id,
            event_type,
            recipients,
            subject,
            body,
            include_link_to_client_invoice,
            attach_pdf,
            send_me_a_copy,
            thank_you,
        }
    }
    /**List all payments for an invoice

Returns a list of payments associate with a given invoice. The payments are returned sorted by creation date, with the most recently created payments appearing first.

The response contains an object with an invoice_payments property that contains an array of up to per_page payments. Each entry in the array is a separate payment object. If no more payments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your payments.

See endpoint docs at <https://help.getharvest.com/api-v2/invoices-api/invoices/invoice-payments/#list-all-payments-for-an-invoice>.*/
    pub fn list_payments_for_invoice(
        &self,
        invoice_id: String,
    ) -> request_model::ListPaymentsForInvoiceRequest {
        request_model::ListPaymentsForInvoiceRequest {
            client: &self,
            invoice_id,
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
        invoice_id: String,
        amount: f64,
        paid_at: String,
        paid_date: String,
        notes: String,
    ) -> request_model::CreateInvoicePaymentRequest {
        request_model::CreateInvoicePaymentRequest {
            client: &self,
            invoice_id,
            amount,
            paid_at,
            paid_date,
            notes,
        }
    }
    /**List all projects

Returns a list of your projects. The projects are returned sorted by creation date, with the most recently created projects appearing first.

The response contains an object with a projects property that contains an array of up to per_page projects. Each entry in the array is a separate project object. If no more projects are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your projects.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#list-all-projects>.*/
    pub fn list_projects(&self) -> request_model::ListProjectsRequest {
        request_model::ListProjectsRequest {
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
    pub fn create_project(
        &self,
        client_id: i64,
        name: String,
        code: String,
        is_active: bool,
        is_billable: bool,
        is_fixed_fee: bool,
        bill_by: String,
        hourly_rate: f64,
        budget: f64,
        budget_by: String,
        budget_is_monthly: bool,
        notify_when_over_budget: bool,
        over_budget_notification_percentage: f64,
        show_budget_to_all: bool,
        cost_budget: f64,
        cost_budget_include_expenses: bool,
        fee: f64,
        notes: String,
        starts_on: String,
        ends_on: String,
    ) -> request_model::CreateProjectRequest {
        request_model::CreateProjectRequest {
            client: &self,
            client_id,
            name,
            code,
            is_active,
            is_billable,
            is_fixed_fee,
            bill_by,
            hourly_rate,
            budget,
            budget_by,
            budget_is_monthly,
            notify_when_over_budget,
            over_budget_notification_percentage,
            show_budget_to_all,
            cost_budget,
            cost_budget_include_expenses,
            fee,
            notes,
            starts_on,
            ends_on,
        }
    }
    /**Retrieve a project

Retrieves the project with the given ID. Returns a project object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/projects/#retrieve-a-project>.*/
    pub fn retrieve_project(
        &self,
        project_id: String,
    ) -> request_model::RetrieveProjectRequest {
        request_model::RetrieveProjectRequest {
            client: &self,
            project_id,
        }
    }
    /**List all task assignments for a specific project

Returns a list of your task assignments for the project identified by PROJECT_ID. The task assignments are returned sorted by creation date, with the most recently created task assignments appearing first.

The response contains an object with a task_assignments property that contains an array of up to per_page task assignments. Each entry in the array is a separate task assignment object. If no more task assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your task assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#list-all-task-assignments-for-a-specific-project>.*/
    pub fn list_task_assignments_for_specific_project(
        &self,
        project_id: String,
    ) -> request_model::ListTaskAssignmentsForSpecificProjectRequest {
        request_model::ListTaskAssignmentsForSpecificProjectRequest {
            client: &self,
            project_id,
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
        project_id: String,
        task_id: i64,
        is_active: bool,
        billable: bool,
        hourly_rate: f64,
        budget: f64,
    ) -> request_model::CreateTaskAssignmentRequest {
        request_model::CreateTaskAssignmentRequest {
            client: &self,
            project_id,
            task_id,
            is_active,
            billable,
            hourly_rate,
            budget,
        }
    }
    /**Retrieve a task assignment

Retrieves the task assignment with the given ID. Returns a task assignment object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#retrieve-a-task-assignment>.*/
    pub fn retrieve_task_assignment(
        &self,
        project_id: String,
        task_assignment_id: String,
    ) -> request_model::RetrieveTaskAssignmentRequest {
        request_model::RetrieveTaskAssignmentRequest {
            client: &self,
            project_id,
            task_assignment_id,
        }
    }
    /**List all user assignments for a specific project

Returns a list of user assignments for the project identified by PROJECT_ID. The user assignments are returned sorted by creation date, with the most recently created user assignments appearing first.

The response contains an object with a user_assignments property that contains an array of up to per_page user assignments. Each entry in the array is a separate user assignment object. If no more user assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your user assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#list-all-user-assignments-for-a-specific-project>.*/
    pub fn list_user_assignments_for_specific_project(
        &self,
        project_id: String,
    ) -> request_model::ListUserAssignmentsForSpecificProjectRequest {
        request_model::ListUserAssignmentsForSpecificProjectRequest {
            client: &self,
            project_id,
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
        project_id: String,
        user_id: i64,
        is_active: bool,
        is_project_manager: bool,
        use_default_rates: bool,
        hourly_rate: f64,
        budget: f64,
    ) -> request_model::CreateUserAssignmentRequest {
        request_model::CreateUserAssignmentRequest {
            client: &self,
            project_id,
            user_id,
            is_active,
            is_project_manager,
            use_default_rates,
            hourly_rate,
            budget,
        }
    }
    /**Retrieve a user assignment

Retrieves the user assignment with the given ID. Returns a user assignment object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#retrieve-a-user-assignment>.*/
    pub fn retrieve_user_assignment(
        &self,
        project_id: String,
        user_assignment_id: String,
    ) -> request_model::RetrieveUserAssignmentRequest {
        request_model::RetrieveUserAssignmentRequest {
            client: &self,
            project_id,
            user_assignment_id,
        }
    }
    /**Expense Categories Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#expense-categories-report>.*/
    pub fn expense_categories_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::ExpenseCategoriesReportRequest {
        request_model::ExpenseCategoriesReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Clients Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#clients-report>.*/
    pub fn clients_expenses_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::ClientsExpensesReportRequest {
        request_model::ClientsExpensesReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Projects Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#projects-report>.*/
    pub fn projects_expenses_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::ProjectsExpensesReportRequest {
        request_model::ProjectsExpensesReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Team Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/expense-reports/#team-report>.*/
    pub fn team_expenses_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::TeamExpensesReportRequest {
        request_model::TeamExpensesReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Project Budget Report

The response contains an object with a results property that contains an array of up to per_page results. Each entry in the array is a separate result object. If no more results are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your results.

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/project-budget-report/#project-budget-report>.*/
    pub fn project_budget_report(&self) -> request_model::ProjectBudgetReportRequest {
        request_model::ProjectBudgetReportRequest {
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
        from: String,
        to: String,
    ) -> request_model::ClientsTimeReportRequest {
        request_model::ClientsTimeReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Projects Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#projects-report>.*/
    pub fn projects_time_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::ProjectsTimeReportRequest {
        request_model::ProjectsTimeReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Tasks Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#tasks-report>.*/
    pub fn tasks_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::TasksReportRequest {
        request_model::TasksReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**Team Report

See endpoint docs at <https://help.getharvest.com/api-v2/reports-api/reports/time-reports/#team-report>.*/
    pub fn team_time_report(
        &self,
        from: String,
        to: String,
    ) -> request_model::TeamTimeReportRequest {
        request_model::TeamTimeReportRequest {
            client: &self,
            from,
            to,
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
        from: String,
        to: String,
    ) -> request_model::UninvoicedReportRequest {
        request_model::UninvoicedReportRequest {
            client: &self,
            from,
            to,
            page: None,
            per_page: None,
        }
    }
    /**List all roles

Returns a list of roles in the account. The roles are returned sorted by creation date, with the most recently created roles appearing first.

The response contains an object with a roles property that contains an array of up to per_page roles. Each entry in the array is a separate role object. If no more roles are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your roles.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#list-all-roles>.*/
    pub fn list_roles(&self) -> request_model::ListRolesRequest {
        request_model::ListRolesRequest {
            client: &self,
            page: None,
            per_page: None,
        }
    }
    /**Create a role

Creates a new role object. Returns a role object and a 201 Created response code if the call succeeded.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#create-a-role>.*/
    pub fn create_role(
        &self,
        name: String,
        user_ids: Vec<i64>,
    ) -> request_model::CreateRoleRequest {
        request_model::CreateRoleRequest {
            client: &self,
            name,
            user_ids,
        }
    }
    /**Retrieve a role

Retrieves the role with the given ID. Returns a role object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/roles-api/roles/roles/#retrieve-a-role>.*/
    pub fn retrieve_role(&self, role_id: String) -> request_model::RetrieveRoleRequest {
        request_model::RetrieveRoleRequest {
            client: &self,
            role_id,
        }
    }
    /**List all task assignments

Returns a list of your task assignments. The task assignments are returned sorted by creation date, with the most recently created task assignments appearing first.

The response contains an object with a task_assignments property that contains an array of up to per_page task assignments. Each entry in the array is a separate task assignment object. If no more task assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your task assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/task-assignments/#list-all-task-assignments>.*/
    pub fn list_task_assignments(&self) -> request_model::ListTaskAssignmentsRequest {
        request_model::ListTaskAssignmentsRequest {
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
    pub fn list_tasks(&self) -> request_model::ListTasksRequest {
        request_model::ListTasksRequest {
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
    pub fn create_task(
        &self,
        name: String,
        billable_by_default: bool,
        default_hourly_rate: f64,
        is_default: bool,
        is_active: bool,
    ) -> request_model::CreateTaskRequest {
        request_model::CreateTaskRequest {
            client: &self,
            name,
            billable_by_default,
            default_hourly_rate,
            is_default,
            is_active,
        }
    }
    /**Retrieve a task

Retrieves the task with the given ID. Returns a task object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/tasks-api/tasks/tasks/#retrieve-a-task>.*/
    pub fn retrieve_task(&self, task_id: String) -> request_model::RetrieveTaskRequest {
        request_model::RetrieveTaskRequest {
            client: &self,
            task_id,
        }
    }
    /**List all time entries

Returns a list of time entries. The time entries are returned sorted by spent_date date. At this time, the sort option can’t be customized.

The response contains an object with a time_entries property that contains an array of up to per_page time entries. Each entry in the array is a separate time entry object. If no more time entries are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your time entries.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#list-all-time-entries>.*/
    pub fn list_time_entries(&self) -> request_model::ListTimeEntriesRequest {
        request_model::ListTimeEntriesRequest {
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
    pub fn create_time_entry(
        &self,
        user_id: i64,
        project_id: i64,
        task_id: i64,
        spent_date: String,
        started_time: String,
        ended_time: String,
        notes: String,
        hours: f64,
    ) -> request_model::CreateTimeEntryRequest {
        request_model::CreateTimeEntryRequest {
            client: &self,
            user_id,
            project_id,
            task_id,
            spent_date,
            started_time,
            ended_time,
            notes,
            external_reference: None,
            hours,
        }
    }
    /**Retrieve a time entry

Retrieves the time entry with the given ID. Returns a time entry object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/timesheets-api/timesheets/time-entries/#retrieve-a-time-entry>.*/
    pub fn retrieve_time_entry(
        &self,
        time_entry_id: String,
    ) -> request_model::RetrieveTimeEntryRequest {
        request_model::RetrieveTimeEntryRequest {
            client: &self,
            time_entry_id,
        }
    }
    /**List all user assignments

Returns a list of your projects user assignments, active and archived. The user assignments are returned sorted by creation date, with the most recently created user assignments appearing first.

The response contains an object with a user_assignments property that contains an array of up to per_page user assignments. Each entry in the array is a separate user assignment object. If no more user assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your user assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/projects-api/projects/user-assignments/#list-all-user-assignments>.*/
    pub fn list_user_assignments(&self) -> request_model::ListUserAssignmentsRequest {
        request_model::ListUserAssignmentsRequest {
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
    pub fn list_users(&self) -> request_model::ListUsersRequest {
        request_model::ListUsersRequest {
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
    pub fn create_user(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        timezone: String,
        has_access_to_all_future_projects: bool,
        is_contractor: bool,
        is_active: bool,
        weekly_capacity: i64,
        default_hourly_rate: f64,
        cost_rate: f64,
        roles: Vec<String>,
    ) -> request_model::CreateUserRequest {
        request_model::CreateUserRequest {
            client: &self,
            first_name,
            last_name,
            email,
            timezone,
            has_access_to_all_future_projects,
            is_contractor,
            is_active,
            weekly_capacity,
            default_hourly_rate,
            cost_rate,
            roles,
        }
    }
    /**Retrieve the currently authenticated user

Retrieves the currently authenticated user. Returns a user object and a 200 OK response code.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#retrieve-the-currently-authenticated-user>.*/
    pub fn retrieve_the_currently_authenticated_user(
        &self,
    ) -> request_model::RetrieveTheCurrentlyAuthenticatedUserRequest {
        request_model::RetrieveTheCurrentlyAuthenticatedUserRequest {
            client: &self,
        }
    }
    /**List active project assignments for the currently authenticated user

Returns a list of your active project assignments for the currently authenticated user. The project assignments are returned sorted by creation date, with the most recently created project assignments appearing first.

The response contains an object with a project_assignments property that contains an array of up to per_page project assignments. Each entry in the array is a separate project assignment object. If no more project assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your project assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/project-assignments/#list-active-project-assignments-for-the-currently-authenticated-user>.*/
    pub fn list_active_project_assignments_for_the_currently_authenticated_user(
        &self,
    ) -> request_model::ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest {
        request_model::ListActiveProjectAssignmentsForTheCurrentlyAuthenticatedUserRequest {
            client: &self,
            page: None,
            per_page: None,
        }
    }
    /**Retrieve a user

Retrieves the user with the given ID. Returns a user object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/users/#retrieve-a-user>.*/
    pub fn retrieve_user(&self, user_id: String) -> request_model::RetrieveUserRequest {
        request_model::RetrieveUserRequest {
            client: &self,
            user_id,
        }
    }
    /**List all billable rates for a specific user

Returns a list of billable rates for the user identified by USER_ID. The billable rates are returned sorted by start_date, with the oldest starting billable rates appearing first.

The response contains an object with a billable_rates property that contains an array of up to per_page billable rates. Each entry in the array is a separate billable rate object. If no more billable rates are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your billable rates.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/billable-rates/#list-all-billable-rates-for-a-specific-user>.*/
    pub fn list_billable_rates_for_specific_user(
        &self,
        user_id: String,
    ) -> request_model::ListBillableRatesForSpecificUserRequest {
        request_model::ListBillableRatesForSpecificUserRequest {
            client: &self,
            user_id,
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
        user_id: String,
        amount: f64,
        start_date: String,
    ) -> request_model::CreateBillableRateRequest {
        request_model::CreateBillableRateRequest {
            client: &self,
            user_id,
            amount,
            start_date,
        }
    }
    /**Retrieve a billable rate

Retrieves the billable rate with the given ID. Returns a billable rate object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/billable-rates/#retrieve-a-billable-rate>.*/
    pub fn retrieve_billable_rate(
        &self,
        user_id: String,
        billable_rate_id: String,
    ) -> request_model::RetrieveBillableRateRequest {
        request_model::RetrieveBillableRateRequest {
            client: &self,
            user_id,
            billable_rate_id,
        }
    }
    /**List all cost rates for a specific user

Returns a list of cost rates for the user identified by USER_ID. The cost rates are returned sorted by start_date, with the oldest starting cost rates appearing first.

The response contains an object with a cost_rates property that contains an array of up to per_page cost rates. Each entry in the array is a separate cost rate object. If no more cost rates are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your cost rates.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/cost-rates/#list-all-cost-rates-for-a-specific-user>.*/
    pub fn list_cost_rates_for_specific_user(
        &self,
        user_id: String,
    ) -> request_model::ListCostRatesForSpecificUserRequest {
        request_model::ListCostRatesForSpecificUserRequest {
            client: &self,
            user_id,
            page: None,
            per_page: None,
        }
    }
    /**Create a cost rate

Creates a new cost rate object. Returns a cost rate object and a 201 Created response code if the call succeeded.


  Creating a cost rate with no start_date will replace a user’s existing rate(s).
  Creating a cost rate with a start_date that is before a user’s existing rate(s) will replace those cost rates with the new one.


See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/cost-rates/#create-a-cost-rate>.*/
    pub fn create_cost_rate(
        &self,
        user_id: String,
        amount: f64,
        start_date: String,
    ) -> request_model::CreateCostRateRequest {
        request_model::CreateCostRateRequest {
            client: &self,
            user_id,
            amount,
            start_date,
        }
    }
    /**Retrieve a cost rate

Retrieves the cost rate with the given ID. Returns a cost rate object and a 200 OK response code if a valid identifier was provided.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/cost-rates/#retrieve-a-cost-rate>.*/
    pub fn retrieve_cost_rate(
        &self,
        user_id: String,
        cost_rate_id: String,
    ) -> request_model::RetrieveCostRateRequest {
        request_model::RetrieveCostRateRequest {
            client: &self,
            user_id,
            cost_rate_id,
        }
    }
    /**List active project assignments

Returns a list of active project assignments for the user identified by USER_ID. The project assignments are returned sorted by creation date, with the most recently created project assignments appearing first.

The response contains an object with a project_assignments property that contains an array of up to per_page project assignments. Each entry in the array is a separate project assignment object. If no more project assignments are available, the resulting array will be empty. Several additional pagination properties are included in the response to simplify paginating your project assignments.

See endpoint docs at <https://help.getharvest.com/api-v2/users-api/users/project-assignments/#list-active-project-assignments>.*/
    pub fn list_active_project_assignments(
        &self,
        user_id: String,
    ) -> request_model::ListActiveProjectAssignmentsRequest {
        request_model::ListActiveProjectAssignmentsRequest {
            client: &self,
            user_id,
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
