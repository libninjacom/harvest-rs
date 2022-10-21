use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceMessageRecipient {
    ///Name of the message recipient.
    pub name: Option<String>,
    ///Email of the message recipient.
    pub email: Option<String>,
}
impl std::fmt::Display for InvoiceMessageRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePayments {
    pub page: i64,
    pub total_pages: i64,
    pub links: PaginationLinks,
    pub per_page: i64,
    pub invoice_payments: Vec<InvoicePayment>,
    pub next_page: Option<i64>,
    pub previous_page: Option<i64>,
    pub total_entries: i64,
}
impl std::fmt::Display for InvoicePayments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    ///The last name of the contact.
    pub last_name: Option<String>,
    ///The contact’s email address.
    pub email: Option<String>,
    ///An object containing the contact’s client id and name.
    pub client: Option<serde_json::Value>,
    ///Unique ID for the contact.
    pub id: Option<i64>,
    ///The title of the contact.
    pub title: Option<String>,
    ///The contact’s office phone number.
    pub phone_office: Option<String>,
    ///The contact’s mobile phone number.
    pub phone_mobile: Option<String>,
    ///The contact’s fax number.
    pub fax: Option<String>,
    ///Date and time the contact was created.
    pub created_at: Option<String>,
    ///Date and time the contact was last updated.
    pub updated_at: Option<String>,
    ///The first name of the contact.
    pub first_name: Option<String>,
}
impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    pub page: i64,
    pub previous_page: Option<i64>,
    pub next_page: Option<i64>,
    pub per_page: i64,
    pub projects: Vec<Project>,
    pub links: PaginationLinks,
    pub total_pages: i64,
    pub total_entries: i64,
}
impl std::fmt::Display for Projects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAssignment {
    ///An object containing the assigned project id, name, and code.
    pub project: Option<serde_json::Value>,
    ///Whether the project assignment is active or archived.
    pub is_active: Option<bool>,
    ///Date and time the project assignment was last updated.
    pub updated_at: Option<String>,
    ///Array of task assignment objects associated with the project.
    pub task_assignments: Option<Vec<TaskAssignment>>,
    ///Date and time the project assignment was created.
    pub created_at: Option<String>,
    ///Determines which billable rate(s) will be used on the project for this user when bill_by is People. When true, the project will use the user’s default billable rates. When false, the project will use the custom rate defined on this user assignment.
    pub use_default_rates: Option<bool>,
    ///Determines if the user has Project Manager permissions for the project.
    pub is_project_manager: Option<bool>,
    ///Budget used when the project’s budget_by is person.
    pub budget: Option<f64>,
    ///An object containing the project’s client id and name.
    pub client: Option<serde_json::Value>,
    ///Custom rate used when the project’s bill_by is People and use_default_rates is false.
    pub hourly_rate: Option<f64>,
    ///Unique ID for the project assignment.
    pub id: Option<i64>,
}
impl std::fmt::Display for ProjectAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignment {
    ///Unique ID for the task assignment.
    pub id: Option<i64>,
    ///An object containing the id and name of the associated task.
    pub task: Option<serde_json::Value>,
    ///An object containing the id, name, and code of the associated project.
    pub project: Option<serde_json::Value>,
    ///Whether the task assignment is billable or not.  For example: if set to true, all time tracked on this project for the associated task will be marked as billable.
    pub billable: Option<bool>,
    ///Date and time the task assignment was last updated.
    pub updated_at: Option<String>,
    ///Budget used when the project’s budget_by is task or task_fees.
    pub budget: Option<f64>,
    ///Rate used when the project’s bill_by is Tasks.
    pub hourly_rate: Option<f64>,
    ///Whether the task assignment is active or archived.
    pub is_active: Option<bool>,
    ///Date and time the task assignment was created.
    pub created_at: Option<String>,
}
impl std::fmt::Display for TaskAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BillableRate {
    ///Date and time the billable rate was last updated.
    pub updated_at: Option<String>,
    ///Date and time the billable rate was created.
    pub created_at: Option<String>,
    ///Unique ID for the billable rate.
    pub id: Option<i64>,
    ///The amount of the billable rate.
    pub amount: Option<f64>,
    ///The date the billable rate is no longer effective. This date is calculated by Harvest.
    pub end_date: Option<String>,
    ///The date the billable rate is effective.
    pub start_date: Option<String>,
}
impl std::fmt::Display for BillableRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMessageRecipients {
    pub total_entries: i64,
    pub previous_page: Option<i64>,
    pub page: i64,
    pub links: PaginationLinks,
    pub invoice_message_recipients: Vec<InvoiceMessageRecipient>,
    pub total_pages: i64,
    pub next_page: Option<i64>,
    pub per_page: i64,
}
impl std::fmt::Display for InvoiceMessageRecipients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Companies {
    pub companies: Vec<Company>,
    pub page: i64,
    pub links: PaginationLinks,
    pub next_page: Option<i64>,
    pub previous_page: Option<i64>,
    pub total_pages: i64,
    pub total_entries: i64,
    pub per_page: i64,
}
impl std::fmt::Display for Companies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Estimates {
    pub total_entries: i64,
    pub per_page: i64,
    pub previous_page: Option<i64>,
    pub links: PaginationLinks,
    pub next_page: Option<i64>,
    pub page: i64,
    pub estimates: Vec<Estimate>,
    pub total_pages: i64,
}
impl std::fmt::Display for Estimates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Expenses {
    pub previous_page: Option<i64>,
    pub page: i64,
    pub total_entries: i64,
    pub links: PaginationLinks,
    pub expenses: Vec<Expense>,
    pub per_page: i64,
    pub total_pages: i64,
    pub next_page: Option<i64>,
}
impl std::fmt::Display for Expenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UninvoicedReportResult {
    ///The name of the project associated with the reported hours and expenses.
    pub project_name: Option<String>,
    ///The total hours for the given timeframe and project. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub total_hours: Option<f64>,
    ///The total hours for the given timeframe and project that have not been invoiced. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub uninvoiced_hours: Option<f64>,
    ///The name of the client associated with the reported hours and expenses.
    pub client_name: Option<String>,
    ///The currency code associated with the tracked hours for this result.
    pub currency: Option<String>,
    ///The ID of the project associated with the reported hours and expenses.
    pub project_id: Option<i64>,
    ///The total amount (time and expenses) for the timeframe and project that have not been invoiced.
    pub uninvoiced_amount: Option<f64>,
    ///The ID of the client associated with the reported hours and expenses.
    pub client_id: Option<i64>,
    ///The total amount for billable expenses for the timeframe and project that have not been invoiced.
    pub uninvoiced_expenses: Option<f64>,
}
impl std::fmt::Display for UninvoicedReportResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillableRates {
    pub total_pages: i64,
    pub links: PaginationLinks,
    pub per_page: i64,
    pub billable_rates: Vec<BillableRate>,
    pub page: i64,
    pub next_page: Option<i64>,
    pub previous_page: Option<i64>,
    pub total_entries: i64,
}
impl std::fmt::Display for BillableRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Error {
    pub code: Option<i64>,
    pub message: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateLineItems {
    pub page: i64,
    pub total_entries: i64,
    pub links: PaginationLinks,
    pub estimate_line_items: Vec<EstimateLineItem>,
    pub per_page: i64,
    pub next_page: Option<i64>,
    pub total_pages: i64,
    pub previous_page: Option<i64>,
}
impl std::fmt::Display for EstimateLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItemCategories {
    pub total_pages: i64,
    pub invoice_item_categories: Vec<InvoiceItemCategory>,
    pub next_page: Option<i64>,
    pub previous_page: Option<i64>,
    pub page: i64,
    pub total_entries: i64,
    pub per_page: i64,
    pub links: PaginationLinks,
}
impl std::fmt::Display for InvoiceItemCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CostRate {
    ///Date and time the cost rate was last updated.
    pub updated_at: Option<String>,
    ///The amount of the cost rate.
    pub amount: Option<f64>,
    ///Unique ID for the cost rate.
    pub id: Option<i64>,
    ///The date the cost rate is effective.
    pub start_date: Option<String>,
    ///The date the cost rate is no longer effective. This date is calculated by Harvest.
    pub end_date: Option<String>,
    ///Date and time the cost rate was created.
    pub created_at: Option<String>,
}
impl std::fmt::Display for CostRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoices {
    pub per_page: i64,
    pub total_entries: i64,
    pub previous_page: Option<i64>,
    pub invoices: Vec<Invoice>,
    pub links: PaginationLinks,
    pub total_pages: i64,
    pub next_page: Option<i64>,
    pub page: i64,
}
impl std::fmt::Display for Invoices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAssignments {
    pub page: i64,
    pub project_assignments: Vec<ProjectAssignment>,
    pub next_page: Option<i64>,
    pub total_pages: i64,
    pub previous_page: Option<i64>,
    pub per_page: i64,
    pub links: PaginationLinks,
    pub total_entries: i64,
}
impl std::fmt::Display for ProjectAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    pub total_pages: i64,
    pub per_page: i64,
    pub page: i64,
    pub links: PaginationLinks,
    pub users: Vec<User>,
    pub next_page: Option<i64>,
    pub previous_page: Option<i64>,
    pub total_entries: i64,
}
impl std::fmt::Display for Users {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePayment {
    ///The name of the person who recorded the payment.
    pub recorded_by: Option<String>,
    ///The email of the person who recorded the payment.
    pub recorded_by_email: Option<String>,
    ///Date and time the payment was recorded.
    pub created_at: Option<String>,
    ///Unique ID for the payment.
    pub id: Option<i64>,
    ///The payment gateway id and name used to process the payment.
    pub payment_gateway: Option<serde_json::Value>,
    ///Date and time the payment was last updated.
    pub updated_at: Option<String>,
    ///Date and time the payment was made.
    pub paid_at: Option<String>,
    ///Date the payment was made.
    pub paid_date: Option<String>,
    ///Any notes associated with the payment.
    pub notes: Option<String>,
    ///Either the card authorization or PayPal transaction ID.
    pub transaction_id: Option<String>,
    ///The amount of the payment.
    pub amount: Option<f64>,
}
impl std::fmt::Display for InvoicePayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignment {
    ///Date and time the user assignment was created.
    pub created_at: Option<String>,
    ///An object containing the id, name, and code of the associated project.
    pub project: Option<serde_json::Value>,
    ///Date and time the user assignment was last updated.
    pub updated_at: Option<String>,
    ///Budget used when the project’s budget_by is person.
    pub budget: Option<f64>,
    ///An object containing the id and name of the associated user.
    pub user: Option<serde_json::Value>,
    ///Whether the user assignment is active or archived.
    pub is_active: Option<bool>,
    ///Determines which billable rate(s) will be used on the project for this user when bill_by is People. When true, the project will use the user’s default billable rates. When false, the project will use the custom rate defined on this user assignment.
    pub use_default_rates: Option<bool>,
    ///Custom rate used when the project’s bill_by is People and use_default_rates is false.
    pub hourly_rate: Option<f64>,
    ///Unique ID for the user assignment.
    pub id: Option<i64>,
    ///Determines if the user has Project Manager permissions for the project.
    pub is_project_manager: Option<bool>,
}
impl std::fmt::Display for UserAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EstimateItemCategory {
    ///Unique ID for the estimate item category.
    pub id: Option<i64>,
    ///Date and time the estimate item category was last updated.
    pub updated_at: Option<String>,
    ///Date and time the estimate item category was created.
    pub created_at: Option<String>,
    ///The name of the estimate item category.
    pub name: Option<String>,
}
impl std::fmt::Display for EstimateItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    ///The amount you plan to invoice for the project. Only used by fixed-fee projects.
    pub fee: Option<f64>,
    ///The method by which the project is budgeted.
    pub budget_by: Option<String>,
    ///Date and time the project was created.
    pub created_at: Option<String>,
    ///Option to show project budget to all employees. Does not apply to Total Project Fee projects.
    pub show_budget_to_all: Option<bool>,
    ///Project notes.
    pub notes: Option<String>,
    ///Unique name for the project.
    pub name: Option<String>,
    ///Option for budget of Total Project Fees projects to include tracked expenses.
    pub cost_budget_include_expenses: Option<bool>,
    ///Date and time the project was last updated.
    pub updated_at: Option<String>,
    ///Date the project will end.
    pub ends_on: Option<String>,
    ///Whether the project is billable or not.
    pub is_billable: Option<bool>,
    ///Whether the project is a fixed-fee project or not.
    pub is_fixed_fee: Option<bool>,
    ///Option to have the budget reset every month.
    pub budget_is_monthly: Option<bool>,
    ///Whether Project Managers should be notified when the project goes over budget.
    pub notify_when_over_budget: Option<bool>,
    ///The code associated with the project.
    pub code: Option<String>,
    ///Whether the project is active or archived.
    pub is_active: Option<bool>,
    ///The monetary budget for the project when budgeting by money.
    pub cost_budget: Option<f64>,
    ///An object containing the project’s client id, name, and currency.
    pub client: Option<serde_json::Value>,
    ///Rate for projects billed by Project Hourly Rate.
    pub hourly_rate: Option<f64>,
    ///The budget in hours for the project when budgeting by time.
    pub budget: Option<f64>,
    ///The method by which the project is invoiced.
    pub bill_by: Option<String>,
    ///Unique ID for the project.
    pub id: Option<i64>,
    ///Percentage value used to trigger over budget email alerts.
    pub over_budget_notification_percentage: Option<f64>,
    ///Date of last over budget notification. If none have been sent, this will be null.
    pub over_budget_notification_date: Option<String>,
    ///Date the project was started.
    pub starts_on: Option<String>,
}
impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseReportsResults {
    pub page: i64,
    pub previous_page: Option<i64>,
    pub total_pages: i64,
    pub results: Vec<ExpenseReportsResult>,
    pub per_page: i64,
    pub total_entries: i64,
    pub links: PaginationLinks,
    pub next_page: Option<i64>,
}
impl std::fmt::Display for ExpenseReportsResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectBudgetReportResults {
    pub page: i64,
    pub per_page: i64,
    pub total_entries: i64,
    pub previous_page: Option<i64>,
    pub links: PaginationLinks,
    pub total_pages: i64,
    pub results: Vec<ProjectBudgetReportResult>,
    pub next_page: Option<i64>,
}
impl std::fmt::Display for ProjectBudgetReportResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CostRates {
    pub previous_page: Option<i64>,
    pub total_pages: i64,
    pub next_page: Option<i64>,
    pub per_page: i64,
    pub page: i64,
    pub links: PaginationLinks,
    pub cost_rates: Vec<CostRate>,
    pub total_entries: i64,
}
impl std::fmt::Display for CostRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceItemCategory {
    ///Unique ID for the invoice item category.
    pub id: Option<i64>,
    ///Date and time the invoice item category was created.
    pub created_at: Option<String>,
    ///Whether this invoice item category is used for expenses when generating an invoice.
    pub use_as_expense: Option<bool>,
    ///Date and time the invoice item category was last updated.
    pub updated_at: Option<String>,
    ///The name of the invoice item category.
    pub name: Option<String>,
    ///Whether this invoice item category is used for billable hours when generating an invoice.
    pub use_as_service: Option<bool>,
}
impl std::fmt::Display for InvoiceItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeReportsResults {
    pub previous_page: Option<i64>,
    pub page: i64,
    pub total_entries: i64,
    pub results: Vec<TimeReportsResult>,
    pub links: PaginationLinks,
    pub per_page: i64,
    pub total_pages: i64,
    pub next_page: Option<i64>,
}
impl std::fmt::Display for TimeReportsResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignments {
    pub links: PaginationLinks,
    pub user_assignments: Vec<UserAssignment>,
    pub next_page: Option<i64>,
    pub page: i64,
    pub per_page: i64,
    pub total_entries: i64,
    pub previous_page: Option<i64>,
    pub total_pages: i64,
}
impl std::fmt::Display for UserAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    ///Unique ID for the invoice.
    pub id: Option<i64>,
    ///The currency code associated with this invoice.
    pub currency: Option<String>,
    ///Unique ID of the associated recurring invoice.
    pub recurring_invoice_id: Option<i64>,
    ///The total amount for the invoice, including any discounts and taxes.
    pub amount: Option<f64>,
    ///Date and time the invoice was sent.
    pub sent_at: Option<String>,
    ///Start of the period during which time entries were added to this invoice.
    pub period_start: Option<String>,
    ///The timeframe in which the invoice should be paid. Options: upon receipt, net 15, net 30, net 45, net 60, or custom.
    pub payment_term: Option<String>,
    ///The current state of the invoice: draft, open, paid, or closed.
    pub state: Option<String>,
    ///Date and time the invoice was created.
    pub created_at: Option<String>,
    ///An object containing the associated estimate’s id.
    pub estimate: Option<serde_json::Value>,
    ///End of the period during which time entries were added to this invoice.
    pub period_end: Option<String>,
    ///An object containing invoice’s client id and name.
    pub client: Option<serde_json::Value>,
    ///Date and time the invoice was paid.
    pub paid_at: Option<String>,
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax: Option<f64>,
    ///The purchase order number.
    pub purchase_order: Option<String>,
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax2: Option<f64>,
    ///The invoice subject.
    pub subject: Option<String>,
    ///An object containing the id and name of the person that created the invoice.
    pub creator: Option<serde_json::Value>,
    ///Date the invoice was paid.
    pub paid_date: Option<String>,
    ///Date the invoice is due.
    pub due_date: Option<String>,
    ///The total amount due at this time for this invoice.
    pub due_amount: Option<f64>,
    ///This percentage is subtracted from the subtotal.
    pub discount: Option<f64>,
    ///The amount calcuated from discount.
    pub discount_amount: Option<f64>,
    ///Array of invoice line items.
    pub line_items: Option<Vec<InvoiceLineItem>>,
    ///Date and time the invoice was last updated.
    pub updated_at: Option<String>,
    ///An object containing the associated retainer’s id.
    pub retainer: Option<serde_json::Value>,
    ///Any additional notes included on the invoice.
    pub notes: Option<String>,
    ///The amount calculated from tax2.
    pub tax2_amount: Option<f64>,
    ///Date and time the invoice was closed.
    pub closed_at: Option<String>,
    ///Used to build a URL to the public web invoice for your client:https://{ACCOUNT_SUBDOMAIN}.harvestapp.com/client/invoices/{CLIENT_KEY}
    pub client_key: Option<String>,
    ///The first amount of tax included, calculated from tax. If no tax is defined, this value will be null.
    pub tax_amount: Option<f64>,
    ///If no value is set, the number will be automatically generated.
    pub number: Option<String>,
    ///Date the invoice was issued.
    pub issue_date: Option<String>,
}
impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMessageRecipients {
    pub page: i64,
    pub links: PaginationLinks,
    pub per_page: i64,
    pub total_entries: i64,
    pub estimate_message_recipients: Vec<EstimateMessageRecipient>,
    pub previous_page: Option<i64>,
    pub total_pages: i64,
    pub next_page: Option<i64>,
}
impl std::fmt::Display for EstimateMessageRecipients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntries {
    pub page: i64,
    pub total_entries: i64,
    pub per_page: i64,
    pub time_entries: Vec<TimeEntry>,
    pub previous_page: Option<i64>,
    pub total_pages: i64,
    pub next_page: Option<i64>,
    pub links: PaginationLinks,
}
impl std::fmt::Display for TimeEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Estimate {
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax: Option<f64>,
    ///An object containing the id and name of the person that created the estimate.
    pub creator: Option<serde_json::Value>,
    ///The first amount of tax included, calculated from tax. If no tax is defined, this value will be null.
    pub tax_amount: Option<f64>,
    ///The estimate subject.
    pub subject: Option<String>,
    ///The current state of the estimate: draft, sent, accepted, or declined.
    pub state: Option<String>,
    ///If no value is set, the number will be automatically generated.
    pub number: Option<String>,
    ///Date and time the estimate was created.
    pub created_at: Option<String>,
    ///Date and time the estimate was last updated.
    pub updated_at: Option<String>,
    ///Date and time the estimate was sent.
    pub sent_at: Option<String>,
    ///Any additional notes included on the estimate.
    pub notes: Option<String>,
    ///Date and time the estimate was accepted.
    pub accepted_at: Option<String>,
    ///The amount calculated from tax2.
    pub tax2_amount: Option<f64>,
    ///Date the estimate was issued.
    pub issue_date: Option<String>,
    ///The currency code associated with this estimate.
    pub currency: Option<String>,
    ///The total amount for the estimate, including any discounts and taxes.
    pub amount: Option<f64>,
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax2: Option<f64>,
    ///The amount calcuated from discount.
    pub discount_amount: Option<f64>,
    ///Date and time the estimate was declined.
    pub declined_at: Option<String>,
    ///An object containing estimate’s client id and name.
    pub client: Option<serde_json::Value>,
    ///This percentage is subtracted from the subtotal.
    pub discount: Option<f64>,
    ///Unique ID for the estimate.
    pub id: Option<i64>,
    ///Array of estimate line items.
    pub line_items: Option<Vec<EstimateLineItem>>,
    ///The purchase order number.
    pub purchase_order: Option<String>,
    ///Used to build a URL to the public web invoice for your client:https://{ACCOUNT_SUBDOMAIN}.harvestapp.com/client/estimates/abc123456
    pub client_key: Option<String>,
}
impl std::fmt::Display for Estimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Contacts {
    pub per_page: i64,
    pub total_pages: i64,
    pub total_entries: i64,
    pub links: PaginationLinks,
    pub previous_page: Option<i64>,
    pub page: i64,
    pub next_page: Option<i64>,
    pub contacts: Vec<Contact>,
}
impl std::fmt::Display for Contacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct User {
    ///The number of hours per week this person is available to work in seconds, in half hour increments. For example, if a person’s capacity is 35 hours, the API will return 126000 seconds.
    pub weekly_capacity: Option<i64>,
    ///The billable rate to use for this user when they are added to a project.
    pub default_hourly_rate: Option<f64>,
    ///The cost rate to use for this user when calculating a project’s costs vs billable amount.
    pub cost_rate: Option<f64>,
    ///The email address of the user.
    pub email: Option<String>,
    ///Whether the user is active or archived.
    pub is_active: Option<bool>,
    ///The user’s telephone number.
    pub telephone: Option<String>,
    ///The first name of the user.
    pub first_name: Option<String>,
    ///Whether the user is a contractor or an employee.
    pub is_contractor: Option<bool>,
    ///The role names assigned to this person.
    pub roles: Option<Vec<String>>,
    ///Whether the user has Project Manager permissions.
    pub is_project_manager: Option<bool>,
    ///The last name of the user.
    pub last_name: Option<String>,
    ///Date and time the user was last updated.
    pub updated_at: Option<String>,
    ///Date and time the user was created.
    pub created_at: Option<String>,
    ///Whether the user has Admin permissions.
    pub is_admin: Option<bool>,
    ///The user’s timezone.
    pub timezone: Option<String>,
    ///Whether the user should be automatically added to future projects.
    pub has_access_to_all_future_projects: Option<bool>,
    ///The URL to the user’s avatar image.
    pub avatar_url: Option<String>,
    ///Unique ID for the user.
    pub id: Option<i64>,
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLineItems {
    pub per_page: i64,
    pub page: i64,
    pub links: PaginationLinks,
    pub total_pages: i64,
    pub total_entries: i64,
    pub invoice_line_items: Vec<InvoiceLineItem>,
    pub previous_page: Option<i64>,
    pub next_page: Option<i64>,
}
impl std::fmt::Display for InvoiceLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    pub tasks: Vec<Task>,
    pub previous_page: Option<i64>,
    pub links: PaginationLinks,
    pub page: i64,
    pub next_page: Option<i64>,
    pub total_entries: i64,
    pub total_pages: i64,
    pub per_page: i64,
}
impl std::fmt::Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Role {
    ///Unique ID for the role.
    pub id: Option<i64>,
    ///The name of the role.
    pub name: Option<String>,
    ///The IDs of the users assigned to this role.
    pub user_ids: Option<Vec<i64>>,
    ///Date and time the role was last updated.
    pub updated_at: Option<String>,
    ///Date and time the role was created.
    pub created_at: Option<String>,
}
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    pub page: i64,
    pub total_pages: i64,
    pub previous_page: Option<i64>,
    pub per_page: i64,
    pub roles: Vec<Role>,
    pub total_entries: i64,
    pub next_page: Option<i64>,
    pub links: PaginationLinks,
}
impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    ///An object containing the expense’s receipt URL and file name.
    pub receipt: Option<serde_json::Value>,
    ///Whether the expense is billable or not.
    pub billable: Option<bool>,
    ///Whether the expense has been approved or closed for some other reason.
    pub is_closed: Option<bool>,
    ///An object containing the expense’s project id, name, and code.
    pub project: Option<serde_json::Value>,
    ///An object containing the expense’s expense category id, name, unit_price, and unit_name.
    pub expense_category: Option<serde_json::Value>,
    pub user_assignment: Option<UserAssignment>,
    ///Date and time the expense was created.
    pub created_at: Option<String>,
    ///An explanation of why the expense has been locked.
    pub locked_reason: Option<String>,
    ///Once the expense has been invoiced, this field will include the associated invoice’s id and number.
    pub invoice: Option<serde_json::Value>,
    ///Textual notes used to describe the expense.
    pub notes: Option<String>,
    ///Date and time the expense was last updated.
    pub updated_at: Option<String>,
    ///Whether or not the expense has been marked as invoiced.
    pub is_billed: Option<bool>,
    ///Whether the expense has been been invoiced, approved, or the project or person related to the expense is archived.
    pub is_locked: Option<bool>,
    ///Date the expense occurred.
    pub spent_date: Option<String>,
    ///An object containing the expense’s client id, name, and currency.
    pub client: Option<serde_json::Value>,
    ///Unique ID for the expense.
    pub id: Option<i64>,
    ///An object containing the id and name of the user that recorded the expense.
    pub user: Option<serde_json::Value>,
    ///The quantity of units used to calculate the total_cost of the expense.
    pub units: Option<i64>,
    ///The total amount of the expense.
    pub total_cost: Option<f64>,
}
impl std::fmt::Display for Expense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Clients {
    pub page: i64,
    pub total_pages: i64,
    pub clients: Vec<Client>,
    pub links: PaginationLinks,
    pub next_page: Option<i64>,
    pub per_page: i64,
    pub previous_page: Option<i64>,
    pub total_entries: i64,
}
impl std::fmt::Display for Clients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Company {
    ///The name of the company.
    pub name: Option<String>,
    ///How to display the currency symbol when formatting currency. Returns one of: symbol_none, symbol_before, or symbol_after.
    pub currency_symbol_display: Option<String>,
    ///Whether the estimate module is enabled.
    pub estimate_feature: Option<bool>,
    ///The type of plan the company is on. Examples: trial, free, or simple-v4
    pub plan_type: Option<String>,
    ///How to display the currency code when formatting currency. Returns one of: iso_code_none, iso_code_before, or iso_code_after.
    pub currency_code_display: Option<String>,
    ///Symbol used when formatting decimals.
    pub decimal_symbol: Option<String>,
    ///The weekly capacity in seconds.
    pub weekly_capacity: Option<i64>,
    ///The Harvest URL for the company.
    pub base_uri: Option<String>,
    ///The Harvest domain for the company.
    pub full_domain: Option<String>,
    ///Whether time is tracked via duration or start and end times.
    pub wants_timestamp_timers: Option<bool>,
    ///The format used to display time in Harvest. Returns either decimal or hours_minutes.
    pub time_format: Option<String>,
    ///Separator used when formatting numbers.
    pub thousands_separator: Option<String>,
    ///The weekday used as the start of the week. Returns one of: Saturday, Sunday, or Monday.
    pub week_start_day: Option<String>,
    ///The color scheme being used in the Harvest web client.
    pub color_scheme: Option<String>,
    ///Whether the expense module is enabled.
    pub expense_feature: Option<bool>,
    ///Whether the approval module is enabled.
    pub approval_feature: Option<bool>,
    ///Whether the invoice module is enabled.
    pub invoice_feature: Option<bool>,
    ///Whether the company is active or archived.
    pub is_active: Option<bool>,
    ///Used to represent whether the company is using a 12-hour or 24-hour clock. Returns either 12h or 24h.
    pub clock: Option<String>,
    ///The format used to display date in Harvest. Returns one of: %m/%d/%Y, %d/%m/%Y, %Y-%m-%d, %d.%m.%Y,.%Y.%m.%d or %Y/%m/%d.
    pub date_format: Option<String>,
}
impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Task {
    ///The name of the task.
    pub name: Option<String>,
    ///Whether this task is active or archived.
    pub is_active: Option<bool>,
    ///Date and time the task was created.
    pub created_at: Option<String>,
    ///The hourly rate to use for this task when it is added to a project.
    pub default_hourly_rate: Option<f64>,
    ///Date and time the task was last updated.
    pub updated_at: Option<String>,
    ///Unique ID for the task.
    pub id: Option<i64>,
    ///Used in determining whether default tasks should be marked billable when creating a new project.
    pub billable_by_default: Option<bool>,
    ///Whether this task should be automatically added to future projects.
    pub is_default: Option<bool>,
}
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMessages {
    pub total_entries: i64,
    pub previous_page: Option<i64>,
    pub next_page: Option<i64>,
    pub links: PaginationLinks,
    pub estimate_messages: Vec<EstimateMessage>,
    pub per_page: i64,
    pub page: i64,
    pub total_pages: i64,
}
impl std::fmt::Display for EstimateMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EstimateMessageRecipient {
    ///Name of the message recipient.
    pub name: Option<String>,
    ///Email of the message recipient.
    pub email: Option<String>,
}
impl std::fmt::Display for EstimateMessageRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateItemCategories {
    pub per_page: i64,
    pub previous_page: Option<i64>,
    pub total_pages: i64,
    pub links: PaginationLinks,
    pub estimate_item_categories: Vec<EstimateItemCategory>,
    pub next_page: Option<i64>,
    pub page: i64,
    pub total_entries: i64,
}
impl std::fmt::Display for EstimateItemCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProjectBudgetReportResult {
    ///The name of the project.
    pub project_name: Option<String>,
    ///The method by which the project is budgeted. Options: project (Hours Per Project), project_cost (Total Project Fees), task (Hours Per Task), task_fees (Fees Per Task), person (Hours Per Person), none (No Budget).
    pub budget_by: Option<String>,
    ///Whether the project is active or archived.
    pub is_active: Option<bool>,
    ///The budget in hours or money for the project when budgeting by time. If the project is budgeted by money, this value will only be visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub budget: Option<f64>,
    ///The ID of the project.
    pub project_id: Option<i64>,
    ///The total hours or money spent against the project’s budget. If Time Rounding is turned on, the hours will be rounded according to your settings. If the project is budgeted by money, this value will only be visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub budget_spent: Option<f64>,
    ///The total hours or money remaining in the project’s budget. If Time Rounding is turned on, the hours will be rounded according to your settings. If the project is budgeted by money, this value will only be visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub budget_remaining: Option<f64>,
    ///The ID of the client associated with this project.
    pub client_id: Option<i64>,
    ///Whether the budget is reset every month.
    pub budget_is_monthly: Option<bool>,
    ///The name of the client associated with this project.
    pub client_name: Option<String>,
}
impl std::fmt::Display for ProjectBudgetReportResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Client {
    ///Whether the client is active or archived.
    pub is_active: Option<bool>,
    ///The physical address for the client.
    pub address: Option<String>,
    ///Unique ID for the client.
    pub id: Option<i64>,
    ///The currency code associated with this client.
    pub currency: Option<String>,
    ///Date and time the client was created.
    pub created_at: Option<String>,
    ///Used to build a URL to your client’s invoice dashboard:https://{ACCOUNT_SUBDOMAIN}.harvestapp.com/client/statements/{STATEMENT_KEY}
    pub statement_key: Option<String>,
    ///Date and time the client was last updated.
    pub updated_at: Option<String>,
    ///A textual description of the client.
    pub name: Option<String>,
}
impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PaginationLinks {
    ///First page
    pub first: String,
    ///Next page
    pub next: Option<String>,
    ///Previous page
    pub previous: Option<String>,
    ///Last page
    pub last: String,
}
impl std::fmt::Display for PaginationLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseCategories {
    pub previous_page: Option<i64>,
    pub page: i64,
    pub total_pages: i64,
    pub total_entries: i64,
    pub expense_categories: Vec<ExpenseCategory>,
    pub next_page: Option<i64>,
    pub links: PaginationLinks,
    pub per_page: i64,
}
impl std::fmt::Display for ExpenseCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EstimateMessage {
    ///Array of estimate message recipients.
    pub recipients: Option<Vec<EstimateMessageRecipient>>,
    ///Date and time the message was created.
    pub created_at: Option<String>,
    ///Email of the user that created the message.
    pub sent_by_email: Option<String>,
    ///Unique ID for the message.
    pub id: Option<i64>,
    ///Date and time the message was last updated.
    pub updated_at: Option<String>,
    ///Email of the user that message was sent from.
    pub sent_from_email: Option<String>,
    ///The message subject.
    pub subject: Option<String>,
    ///The type of estimate event that occurred with the message: send, accept, decline, re-open, view, or invoice.
    pub event_type: Option<String>,
    ///The message body.
    pub body: Option<String>,
    ///Name of the user that created the message.
    pub sent_by: Option<String>,
    ///Name of the user that the message was sent from.
    pub sent_from: Option<String>,
    ///Whether to email a copy of the message to the current user.
    pub send_me_a_copy: Option<bool>,
}
impl std::fmt::Display for EstimateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    ///The unit quantity of the item.
    pub quantity: Option<f64>,
    ///Text description of the line item.
    pub description: Option<String>,
    ///An object containing the associated project’s id, name, and code.
    pub project: Option<serde_json::Value>,
    ///Whether the invoice’s tax percentage applies to this line item.
    pub taxed: Option<bool>,
    ///Whether the invoice’s tax2 percentage applies to this line item.
    pub taxed2: Option<bool>,
    ///Unique ID for the line item.
    pub id: Option<i64>,
    ///The name of an invoice item category.
    pub kind: Option<String>,
    ///The individual price per unit.
    pub unit_price: Option<f64>,
    ///The line item subtotal (quantity * unit_price).
    pub amount: Option<f64>,
}
impl std::fmt::Display for InvoiceLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMessages {
    pub next_page: Option<i64>,
    pub links: PaginationLinks,
    pub invoice_messages: Vec<InvoiceMessage>,
    pub per_page: i64,
    pub total_pages: i64,
    pub total_entries: i64,
    pub previous_page: Option<i64>,
    pub page: i64,
}
impl std::fmt::Display for InvoiceMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    ///Number of (decimal time) hours tracked in this time entry used in summary reports and invoices. This value is rounded according to the Time Rounding setting in your Preferences.
    pub rounded_hours: Option<f64>,
    ///Number of (decimal time) hours already tracked in this time entry, before the timer was last started.
    pub hours_without_timer: Option<f64>,
    ///Date of the time entry.
    pub spent_date: Option<String>,
    pub user_assignment: Option<UserAssignment>,
    ///An object containing the id and name of the associated project.
    pub project: Option<serde_json::Value>,
    ///Why the time entry has been locked.
    pub locked_reason: Option<String>,
    ///Whether or not the time entry has been locked.
    pub is_locked: Option<bool>,
    ///Whether or not the time entry is currently running.
    pub is_running: Option<bool>,
    ///Whether or not the time entry is billable.
    pub billable: Option<bool>,
    ///Date and time the time entry was created. Use the ISO 8601 Format.
    pub created_at: Option<String>,
    ///Time the time entry was ended (if tracking by start/end times).
    pub ended_time: Option<String>,
    ///The cost rate for the time entry.
    pub cost_rate: Option<f64>,
    ///Number of (decimal time) hours tracked in this time entry.
    pub hours: Option<f64>,
    ///Date and time the time entry was last updated. Use the ISO 8601 Format.
    pub updated_at: Option<String>,
    ///An object containing the id and name of the associated user.
    pub user: Option<serde_json::Value>,
    ///Date and time the timer was started (if tracking by duration). Use the ISO 8601 Format.
    pub timer_started_at: Option<String>,
    ///Whether or not the time entry has been marked as invoiced.
    pub is_billed: Option<bool>,
    ///Notes attached to the time entry.
    pub notes: Option<String>,
    ///Whether or not the time entry has been approved via Timesheet Approval.
    pub is_closed: Option<bool>,
    ///An object containing the id, group_id, account_id, permalink, service, and service_icon_url of the associated external reference.
    pub external_reference: Option<serde_json::Value>,
    ///Once the time entry has been invoiced, this field will include the associated invoice’s id and number.
    pub invoice: Option<serde_json::Value>,
    ///Unique ID for the time entry.
    pub id: Option<i64>,
    ///Time the time entry was started (if tracking by start/end times).
    pub started_time: Option<String>,
    ///Whether or not the time entry counts towards the project budget.
    pub budgeted: Option<bool>,
    pub task_assignment: Option<TaskAssignment>,
    ///An object containing the id and name of the associated task.
    pub task: Option<serde_json::Value>,
    ///An object containing the id and name of the associated client.
    pub client: Option<serde_json::Value>,
    ///The billable rate for the time entry.
    pub billable_rate: Option<f64>,
}
impl std::fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExpenseCategory {
    ///Unique ID for the expense category.
    pub id: Option<i64>,
    ///The unit price of the expense category.
    pub unit_price: Option<f64>,
    ///Whether the expense category is active or archived.
    pub is_active: Option<bool>,
    ///Date and time the expense category was created.
    pub created_at: Option<String>,
    ///The unit name of the expense category.
    pub unit_name: Option<String>,
    ///Date and time the expense category was last updated.
    pub updated_at: Option<String>,
    ///The name of the expense category.
    pub name: Option<String>,
}
impl std::fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct InvoiceMessage {
    ///Name of the user that the message was sent from.
    pub sent_from: Option<String>,
    ///Date and time the message was created.
    pub created_at: Option<String>,
    ///Date and time the message was last updated.
    pub updated_at: Option<String>,
    ///Unique ID for the message.
    pub id: Option<i64>,
    ///Array of invoice message recipients.
    pub recipients: Option<Vec<InvoiceMessageRecipient>>,
    ///The message body.
    pub body: Option<String>,
    ///Email of the user that created the message.
    pub sent_by_email: Option<String>,
    ///Whether to include a link to the client invoice in the message body. Not used when thank_you is true.
    pub include_link_to_client_invoice: Option<bool>,
    ///The message subject.
    pub subject: Option<String>,
    ///Whether to email a copy of the message to the current user.
    pub send_me_a_copy: Option<bool>,
    ///Whether this is a thank you message.
    pub thank_you: Option<bool>,
    ///The date the reminder email will be sent.
    pub send_reminder_on: Option<String>,
    ///Name of the user that created the message.
    pub sent_by: Option<String>,
    ///Whether this is a reminder message.
    pub reminder: Option<bool>,
    ///Email of the user that message was sent from.
    pub sent_from_email: Option<String>,
    ///The type of invoice event that occurred with the message: send, close, draft, re-open, or view.
    pub event_type: Option<String>,
    ///Whether to attach the invoice PDF to the message email.
    pub attach_pdf: Option<bool>,
}
impl std::fmt::Display for InvoiceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UninvoicedReportResults {
    pub results: Vec<UninvoicedReportResult>,
    pub per_page: i64,
    pub total_pages: i64,
    pub previous_page: Option<i64>,
    pub next_page: Option<i64>,
    pub page: i64,
    pub links: PaginationLinks,
    pub total_entries: i64,
}
impl std::fmt::Display for UninvoicedReportResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignments {
    pub next_page: Option<i64>,
    pub previous_page: Option<i64>,
    pub task_assignments: Vec<TaskAssignment>,
    pub per_page: i64,
    pub total_pages: i64,
    pub page: i64,
    pub total_entries: i64,
    pub links: PaginationLinks,
}
impl std::fmt::Display for TaskAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EstimateLineItem {
    ///The name of an estimate item category.
    pub kind: Option<String>,
    ///Whether the estimate’s tax percentage applies to this line item.
    pub taxed: Option<bool>,
    ///Unique ID for the line item.
    pub id: Option<i64>,
    ///The line item subtotal (quantity * unit_price).
    pub amount: Option<f64>,
    ///The individual price per unit.
    pub unit_price: Option<f64>,
    ///The unit quantity of the item.
    pub quantity: Option<f64>,
    ///Whether the estimate’s tax2 percentage applies to this line item.
    pub taxed2: Option<bool>,
    ///Text description of the line item.
    pub description: Option<String>,
}
impl std::fmt::Display for EstimateLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExpenseReportsResult {
    ///The currency code associated with the expenses for this result.
    pub currency: Option<String>,
    ///The totaled cost for billable expenses for the given timeframe, subject (client, project, expense category, or user), and currency.
    pub billable_amount: Option<f64>,
    ///The name of the client associated with the reported expenses. Only returned in the Client and Project reports.
    pub client_name: Option<String>,
    ///The ID of the user associated with the reported expenses. Only returned in the Team report.
    pub user_id: Option<i64>,
    ///The ID of the client associated with the reported expenses. Only returned in the Client and Project reports.
    pub client_id: Option<i64>,
    ///The name of the project associated with the reported expenses. Only returned in the Client and Project reports.
    pub project_name: Option<String>,
    ///The name of the expense category associated with the reported expenses. Only returned in the Expense Category report.
    pub expense_category_name: Option<String>,
    ///The name of the user associated with the reported expenses. Only returned in the Team report.
    pub user_name: Option<String>,
    ///The contractor status of the user associated with the reported expenses. Only returned in the Team report.
    pub is_contractor: Option<bool>,
    ///The totaled cost for all expenses for the given timeframe, subject (client, project, expense category, or user), and currency.
    pub total_amount: Option<f64>,
    ///The ID of the project associated with the reported expenses. Only returned in the Client and Project reports.
    pub project_id: Option<i64>,
    ///The ID of the expense category associated with the reported expenses. Only returned in the Expense Category report.
    pub expense_category_id: Option<i64>,
}
impl std::fmt::Display for ExpenseReportsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TimeReportsResult {
    ///The totaled billable hours for the given timeframe, subject (client, project, task, or user), and currency. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub billable_hours: Option<f64>,
    ///The ID of the client associated with the reported hours. Only returned in the Client and Project reports.
    pub client_id: Option<i64>,
    ///The name of the project associated with the reported hours. Only returned in the Client and Project reports.
    pub project_name: Option<String>,
    ///The ID of the task associated with the reported hours. Only returned in the Task report.
    pub task_id: Option<i64>,
    ///The totaled hours for the given timeframe, subject (client, project, task, or user), and currency. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub total_hours: Option<f64>,
    ///The name of the client associated with the reported hours. Only returned in the Client and Project reports.
    pub client_name: Option<String>,
    ///The name of the task associated with the reported hours. Only returned in the Task report.
    pub task_name: Option<String>,
    ///The ID of the user associated with the reported hours. Only returned in the Team report.
    pub user_id: Option<i64>,
    ///The name of the user associated with the reported hours. Only returned in the Team report.
    pub user_name: Option<String>,
    ///The currency code associated with the tracked hours for this result. Only visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub currency: Option<String>,
    ///The totaled billable amount for the billable hours above. Only visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub billable_amount: Option<f64>,
    ///The ID of the project associated with the reported hours. Only returned in the Client and Project reports.
    pub project_id: Option<i64>,
    ///The contractor status of the user associated with the reported hours. Only returned in the Team report.
    pub is_contractor: Option<bool>,
}
impl std::fmt::Display for TimeReportsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
