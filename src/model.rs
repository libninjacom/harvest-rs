use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    #[serde(rename = "id")]
    ///Unique ID for the contact.
    pub id: Option<i64>,
    #[serde(rename = "client")]
    ///An object containing the contact’s client id and name.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "title")]
    ///The title of the contact.
    pub title: Option<String>,
    #[serde(rename = "first_name")]
    ///The first name of the contact.
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    ///The last name of the contact.
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    ///The contact’s email address.
    pub email: Option<String>,
    #[serde(rename = "phone_office")]
    ///The contact’s office phone number.
    pub phone_office: Option<String>,
    #[serde(rename = "phone_mobile")]
    ///The contact’s mobile phone number.
    pub phone_mobile: Option<String>,
    #[serde(rename = "fax")]
    ///The contact’s fax number.
    pub fax: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the contact was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the contact was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    #[serde(rename = "id")]
    ///Unique ID for the client.
    pub id: Option<i64>,
    #[serde(rename = "name")]
    ///A textual description of the client.
    pub name: Option<String>,
    #[serde(rename = "is_active")]
    ///Whether the client is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "address")]
    ///The physical address for the client.
    pub address: Option<String>,
    #[serde(rename = "statement_key")]
    ///Used to build a URL to your client’s invoice dashboard:https://{ACCOUNT_SUBDOMAIN}.harvestapp.com/client/statements/{STATEMENT_KEY}
    pub statement_key: Option<String>,
    #[serde(rename = "currency")]
    ///The currency code associated with this client.
    pub currency: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the client was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the client was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    #[serde(rename = "base_uri")]
    ///The Harvest URL for the company.
    pub base_uri: Option<String>,
    #[serde(rename = "full_domain")]
    ///The Harvest domain for the company.
    pub full_domain: Option<String>,
    #[serde(rename = "name")]
    ///The name of the company.
    pub name: Option<String>,
    #[serde(rename = "is_active")]
    ///Whether the company is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "week_start_day")]
    ///The weekday used as the start of the week. Returns one of: Saturday, Sunday, or Monday.
    pub week_start_day: Option<String>,
    #[serde(rename = "wants_timestamp_timers")]
    ///Whether time is tracked via duration or start and end times.
    pub wants_timestamp_timers: Option<bool>,
    #[serde(rename = "time_format")]
    ///The format used to display time in Harvest. Returns either decimal or hours_minutes.
    pub time_format: Option<String>,
    #[serde(rename = "date_format")]
    ///The format used to display date in Harvest. Returns one of: %m/%d/%Y, %d/%m/%Y, %Y-%m-%d, %d.%m.%Y,.%Y.%m.%d or %Y/%m/%d.
    pub date_format: Option<String>,
    #[serde(rename = "plan_type")]
    ///The type of plan the company is on. Examples: trial, free, or simple-v4
    pub plan_type: Option<String>,
    #[serde(rename = "clock")]
    ///Used to represent whether the company is using a 12-hour or 24-hour clock. Returns either 12h or 24h.
    pub clock: Option<String>,
    #[serde(rename = "currency_code_display")]
    ///How to display the currency code when formatting currency. Returns one of: iso_code_none, iso_code_before, or iso_code_after.
    pub currency_code_display: Option<String>,
    #[serde(rename = "currency_symbol_display")]
    ///How to display the currency symbol when formatting currency. Returns one of: symbol_none, symbol_before, or symbol_after.
    pub currency_symbol_display: Option<String>,
    #[serde(rename = "decimal_symbol")]
    ///Symbol used when formatting decimals.
    pub decimal_symbol: Option<String>,
    #[serde(rename = "thousands_separator")]
    ///Separator used when formatting numbers.
    pub thousands_separator: Option<String>,
    #[serde(rename = "color_scheme")]
    ///The color scheme being used in the Harvest web client.
    pub color_scheme: Option<String>,
    #[serde(rename = "weekly_capacity")]
    ///The weekly capacity in seconds.
    pub weekly_capacity: Option<i64>,
    #[serde(rename = "expense_feature")]
    ///Whether the expense module is enabled.
    pub expense_feature: Option<bool>,
    #[serde(rename = "invoice_feature")]
    ///Whether the invoice module is enabled.
    pub invoice_feature: Option<bool>,
    #[serde(rename = "estimate_feature")]
    ///Whether the estimate module is enabled.
    pub estimate_feature: Option<bool>,
    #[serde(rename = "approval_feature")]
    ///Whether the approval module is enabled.
    pub approval_feature: Option<bool>,
}
impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMessage {
    #[serde(rename = "id")]
    ///Unique ID for the message.
    pub id: Option<i64>,
    #[serde(rename = "sent_by")]
    ///Name of the user that created the message.
    pub sent_by: Option<String>,
    #[serde(rename = "sent_by_email")]
    ///Email of the user that created the message.
    pub sent_by_email: Option<String>,
    #[serde(rename = "sent_from")]
    ///Name of the user that the message was sent from.
    pub sent_from: Option<String>,
    #[serde(rename = "sent_from_email")]
    ///Email of the user that message was sent from.
    pub sent_from_email: Option<String>,
    #[serde(rename = "recipients")]
    ///Array of invoice message recipients.
    pub recipients: Option<Vec<InvoiceMessageRecipient>>,
    #[serde(rename = "subject")]
    ///The message subject.
    pub subject: Option<String>,
    #[serde(rename = "body")]
    ///The message body.
    pub body: Option<String>,
    #[serde(rename = "include_link_to_client_invoice")]
    ///Whether to include a link to the client invoice in the message body. Not used when thank_you is true.
    pub include_link_to_client_invoice: Option<bool>,
    #[serde(rename = "attach_pdf")]
    ///Whether to attach the invoice PDF to the message email.
    pub attach_pdf: Option<bool>,
    #[serde(rename = "send_me_a_copy")]
    ///Whether to email a copy of the message to the current user.
    pub send_me_a_copy: Option<bool>,
    #[serde(rename = "thank_you")]
    ///Whether this is a thank you message.
    pub thank_you: Option<bool>,
    #[serde(rename = "event_type")]
    ///The type of invoice event that occurred with the message: send, close, draft, re-open, or view.
    pub event_type: Option<String>,
    #[serde(rename = "reminder")]
    ///Whether this is a reminder message.
    pub reminder: Option<bool>,
    #[serde(rename = "send_reminder_on")]
    ///The date the reminder email will be sent.
    pub send_reminder_on: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the message was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the message was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for InvoiceMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMessageRecipient {
    #[serde(rename = "name")]
    ///Name of the message recipient.
    pub name: Option<String>,
    #[serde(rename = "email")]
    ///Email of the message recipient.
    pub email: Option<String>,
}
impl std::fmt::Display for InvoiceMessageRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePayment {
    #[serde(rename = "id")]
    ///Unique ID for the payment.
    pub id: Option<i64>,
    #[serde(rename = "amount")]
    ///The amount of the payment.
    pub amount: Option<f64>,
    #[serde(rename = "paid_at")]
    ///Date and time the payment was made.
    pub paid_at: Option<String>,
    #[serde(rename = "paid_date")]
    ///Date the payment was made.
    pub paid_date: Option<String>,
    #[serde(rename = "recorded_by")]
    ///The name of the person who recorded the payment.
    pub recorded_by: Option<String>,
    #[serde(rename = "recorded_by_email")]
    ///The email of the person who recorded the payment.
    pub recorded_by_email: Option<String>,
    #[serde(rename = "notes")]
    ///Any notes associated with the payment.
    pub notes: Option<String>,
    #[serde(rename = "transaction_id")]
    ///Either the card authorization or PayPal transaction ID.
    pub transaction_id: Option<String>,
    #[serde(rename = "payment_gateway")]
    ///The payment gateway id and name used to process the payment.
    pub payment_gateway: Option<serde_json::Value>,
    #[serde(rename = "created_at")]
    ///Date and time the payment was recorded.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the payment was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for InvoicePayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoice {
    #[serde(rename = "id")]
    ///Unique ID for the invoice.
    pub id: Option<i64>,
    #[serde(rename = "client")]
    ///An object containing invoice’s client id and name.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "line_items")]
    ///Array of invoice line items.
    pub line_items: Option<Vec<InvoiceLineItem>>,
    #[serde(rename = "estimate")]
    ///An object containing the associated estimate’s id.
    pub estimate: Option<serde_json::Value>,
    #[serde(rename = "retainer")]
    ///An object containing the associated retainer’s id.
    pub retainer: Option<serde_json::Value>,
    #[serde(rename = "creator")]
    ///An object containing the id and name of the person that created the invoice.
    pub creator: Option<serde_json::Value>,
    #[serde(rename = "client_key")]
    ///Used to build a URL to the public web invoice for your client:https://{ACCOUNT_SUBDOMAIN}.harvestapp.com/client/invoices/{CLIENT_KEY}
    pub client_key: Option<String>,
    #[serde(rename = "number")]
    ///If no value is set, the number will be automatically generated.
    pub number: Option<String>,
    #[serde(rename = "purchase_order")]
    ///The purchase order number.
    pub purchase_order: Option<String>,
    #[serde(rename = "amount")]
    ///The total amount for the invoice, including any discounts and taxes.
    pub amount: Option<f64>,
    #[serde(rename = "due_amount")]
    ///The total amount due at this time for this invoice.
    pub due_amount: Option<f64>,
    #[serde(rename = "tax")]
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax: Option<f64>,
    #[serde(rename = "tax_amount")]
    ///The first amount of tax included, calculated from tax. If no tax is defined, this value will be null.
    pub tax_amount: Option<f64>,
    #[serde(rename = "tax2")]
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax_2: Option<f64>,
    #[serde(rename = "tax2_amount")]
    ///The amount calculated from tax2.
    pub tax_2_amount: Option<f64>,
    #[serde(rename = "discount")]
    ///This percentage is subtracted from the subtotal.
    pub discount: Option<f64>,
    #[serde(rename = "discount_amount")]
    ///The amount calcuated from discount.
    pub discount_amount: Option<f64>,
    #[serde(rename = "subject")]
    ///The invoice subject.
    pub subject: Option<String>,
    #[serde(rename = "notes")]
    ///Any additional notes included on the invoice.
    pub notes: Option<String>,
    #[serde(rename = "currency")]
    ///The currency code associated with this invoice.
    pub currency: Option<String>,
    #[serde(rename = "state")]
    ///The current state of the invoice: draft, open, paid, or closed.
    pub state: Option<String>,
    #[serde(rename = "period_start")]
    ///Start of the period during which time entries were added to this invoice.
    pub period_start: Option<String>,
    #[serde(rename = "period_end")]
    ///End of the period during which time entries were added to this invoice.
    pub period_end: Option<String>,
    #[serde(rename = "issue_date")]
    ///Date the invoice was issued.
    pub issue_date: Option<String>,
    #[serde(rename = "due_date")]
    ///Date the invoice is due.
    pub due_date: Option<String>,
    #[serde(rename = "payment_term")]
    ///The timeframe in which the invoice should be paid. Options: upon receipt, net 15, net 30, net 45, net 60, or custom.
    pub payment_term: Option<String>,
    #[serde(rename = "sent_at")]
    ///Date and time the invoice was sent.
    pub sent_at: Option<String>,
    #[serde(rename = "paid_at")]
    ///Date and time the invoice was paid.
    pub paid_at: Option<String>,
    #[serde(rename = "paid_date")]
    ///Date the invoice was paid.
    pub paid_date: Option<String>,
    #[serde(rename = "closed_at")]
    ///Date and time the invoice was closed.
    pub closed_at: Option<String>,
    #[serde(rename = "recurring_invoice_id")]
    ///Unique ID of the associated recurring invoice.
    pub recurring_invoice_id: Option<i64>,
    #[serde(rename = "created_at")]
    ///Date and time the invoice was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the invoice was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLineItem {
    #[serde(rename = "id")]
    ///Unique ID for the line item.
    pub id: Option<i64>,
    #[serde(rename = "project")]
    ///An object containing the associated project’s id, name, and code.
    pub project: Option<serde_json::Value>,
    #[serde(rename = "kind")]
    ///The name of an invoice item category.
    pub kind: Option<String>,
    #[serde(rename = "description")]
    ///Text description of the line item.
    pub description: Option<String>,
    #[serde(rename = "quantity")]
    ///The unit quantity of the item.
    pub quantity: Option<f64>,
    #[serde(rename = "unit_price")]
    ///The individual price per unit.
    pub unit_price: Option<f64>,
    #[serde(rename = "amount")]
    ///The line item subtotal (quantity * unit_price).
    pub amount: Option<f64>,
    #[serde(rename = "taxed")]
    ///Whether the invoice’s tax percentage applies to this line item.
    pub taxed: Option<bool>,
    #[serde(rename = "taxed2")]
    ///Whether the invoice’s tax2 percentage applies to this line item.
    pub taxed_2: Option<bool>,
}
impl std::fmt::Display for InvoiceLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItemCategory {
    #[serde(rename = "id")]
    ///Unique ID for the invoice item category.
    pub id: Option<i64>,
    #[serde(rename = "name")]
    ///The name of the invoice item category.
    pub name: Option<String>,
    #[serde(rename = "use_as_service")]
    ///Whether this invoice item category is used for billable hours when generating an invoice.
    pub use_as_service: Option<bool>,
    #[serde(rename = "use_as_expense")]
    ///Whether this invoice item category is used for expenses when generating an invoice.
    pub use_as_expense: Option<bool>,
    #[serde(rename = "created_at")]
    ///Date and time the invoice item category was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the invoice item category was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for InvoiceItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMessage {
    #[serde(rename = "id")]
    ///Unique ID for the message.
    pub id: Option<i64>,
    #[serde(rename = "sent_by")]
    ///Name of the user that created the message.
    pub sent_by: Option<String>,
    #[serde(rename = "sent_by_email")]
    ///Email of the user that created the message.
    pub sent_by_email: Option<String>,
    #[serde(rename = "sent_from")]
    ///Name of the user that the message was sent from.
    pub sent_from: Option<String>,
    #[serde(rename = "sent_from_email")]
    ///Email of the user that message was sent from.
    pub sent_from_email: Option<String>,
    #[serde(rename = "recipients")]
    ///Array of estimate message recipients.
    pub recipients: Option<Vec<EstimateMessageRecipient>>,
    #[serde(rename = "subject")]
    ///The message subject.
    pub subject: Option<String>,
    #[serde(rename = "body")]
    ///The message body.
    pub body: Option<String>,
    #[serde(rename = "send_me_a_copy")]
    ///Whether to email a copy of the message to the current user.
    pub send_me_a_copy: Option<bool>,
    #[serde(rename = "event_type")]
    ///The type of estimate event that occurred with the message: send, accept, decline, re-open, view, or invoice.
    pub event_type: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the message was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the message was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for EstimateMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMessageRecipient {
    #[serde(rename = "name")]
    ///Name of the message recipient.
    pub name: Option<String>,
    #[serde(rename = "email")]
    ///Email of the message recipient.
    pub email: Option<String>,
}
impl std::fmt::Display for EstimateMessageRecipient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Estimate {
    #[serde(rename = "id")]
    ///Unique ID for the estimate.
    pub id: Option<i64>,
    #[serde(rename = "client")]
    ///An object containing estimate’s client id and name.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "line_items")]
    ///Array of estimate line items.
    pub line_items: Option<Vec<EstimateLineItem>>,
    #[serde(rename = "creator")]
    ///An object containing the id and name of the person that created the estimate.
    pub creator: Option<serde_json::Value>,
    #[serde(rename = "client_key")]
    ///Used to build a URL to the public web invoice for your client:https://{ACCOUNT_SUBDOMAIN}.harvestapp.com/client/estimates/abc123456
    pub client_key: Option<String>,
    #[serde(rename = "number")]
    ///If no value is set, the number will be automatically generated.
    pub number: Option<String>,
    #[serde(rename = "purchase_order")]
    ///The purchase order number.
    pub purchase_order: Option<String>,
    #[serde(rename = "amount")]
    ///The total amount for the estimate, including any discounts and taxes.
    pub amount: Option<f64>,
    #[serde(rename = "tax")]
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax: Option<f64>,
    #[serde(rename = "tax_amount")]
    ///The first amount of tax included, calculated from tax. If no tax is defined, this value will be null.
    pub tax_amount: Option<f64>,
    #[serde(rename = "tax2")]
    ///This percentage is applied to the subtotal, including line items and discounts.
    pub tax_2: Option<f64>,
    #[serde(rename = "tax2_amount")]
    ///The amount calculated from tax2.
    pub tax_2_amount: Option<f64>,
    #[serde(rename = "discount")]
    ///This percentage is subtracted from the subtotal.
    pub discount: Option<f64>,
    #[serde(rename = "discount_amount")]
    ///The amount calcuated from discount.
    pub discount_amount: Option<f64>,
    #[serde(rename = "subject")]
    ///The estimate subject.
    pub subject: Option<String>,
    #[serde(rename = "notes")]
    ///Any additional notes included on the estimate.
    pub notes: Option<String>,
    #[serde(rename = "currency")]
    ///The currency code associated with this estimate.
    pub currency: Option<String>,
    #[serde(rename = "state")]
    ///The current state of the estimate: draft, sent, accepted, or declined.
    pub state: Option<String>,
    #[serde(rename = "issue_date")]
    ///Date the estimate was issued.
    pub issue_date: Option<String>,
    #[serde(rename = "sent_at")]
    ///Date and time the estimate was sent.
    pub sent_at: Option<String>,
    #[serde(rename = "accepted_at")]
    ///Date and time the estimate was accepted.
    pub accepted_at: Option<String>,
    #[serde(rename = "declined_at")]
    ///Date and time the estimate was declined.
    pub declined_at: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the estimate was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the estimate was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Estimate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateLineItem {
    #[serde(rename = "id")]
    ///Unique ID for the line item.
    pub id: Option<i64>,
    #[serde(rename = "kind")]
    ///The name of an estimate item category.
    pub kind: Option<String>,
    #[serde(rename = "description")]
    ///Text description of the line item.
    pub description: Option<String>,
    #[serde(rename = "quantity")]
    ///The unit quantity of the item.
    pub quantity: Option<f64>,
    #[serde(rename = "unit_price")]
    ///The individual price per unit.
    pub unit_price: Option<f64>,
    #[serde(rename = "amount")]
    ///The line item subtotal (quantity * unit_price).
    pub amount: Option<f64>,
    #[serde(rename = "taxed")]
    ///Whether the estimate’s tax percentage applies to this line item.
    pub taxed: Option<bool>,
    #[serde(rename = "taxed2")]
    ///Whether the estimate’s tax2 percentage applies to this line item.
    pub taxed_2: Option<bool>,
}
impl std::fmt::Display for EstimateLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateItemCategory {
    #[serde(rename = "id")]
    ///Unique ID for the estimate item category.
    pub id: Option<i64>,
    #[serde(rename = "name")]
    ///The name of the estimate item category.
    pub name: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the estimate item category was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the estimate item category was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for EstimateItemCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Expense {
    #[serde(rename = "id")]
    ///Unique ID for the expense.
    pub id: Option<i64>,
    #[serde(rename = "client")]
    ///An object containing the expense’s client id, name, and currency.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "project")]
    ///An object containing the expense’s project id, name, and code.
    pub project: Option<serde_json::Value>,
    #[serde(rename = "expense_category")]
    ///An object containing the expense’s expense category id, name, unit_price, and unit_name.
    pub expense_category: Option<serde_json::Value>,
    #[serde(rename = "user")]
    ///An object containing the id and name of the user that recorded the expense.
    pub user: Option<serde_json::Value>,
    #[serde(rename = "user_assignment")]
    pub user_assignment: Option<UserAssignment>,
    #[serde(rename = "receipt")]
    ///An object containing the expense’s receipt URL and file name.
    pub receipt: Option<serde_json::Value>,
    #[serde(rename = "invoice")]
    ///Once the expense has been invoiced, this field will include the associated invoice’s id and number.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "notes")]
    ///Textual notes used to describe the expense.
    pub notes: Option<String>,
    #[serde(rename = "units")]
    ///The quantity of units used to calculate the total_cost of the expense.
    pub units: Option<i64>,
    #[serde(rename = "total_cost")]
    ///The total amount of the expense.
    pub total_cost: Option<f64>,
    #[serde(rename = "billable")]
    ///Whether the expense is billable or not.
    pub billable: Option<bool>,
    #[serde(rename = "is_closed")]
    ///Whether the expense has been approved or closed for some other reason.
    pub is_closed: Option<bool>,
    #[serde(rename = "is_locked")]
    ///Whether the expense has been been invoiced, approved, or the project or person related to the expense is archived.
    pub is_locked: Option<bool>,
    #[serde(rename = "is_billed")]
    ///Whether or not the expense has been marked as invoiced.
    pub is_billed: Option<bool>,
    #[serde(rename = "locked_reason")]
    ///An explanation of why the expense has been locked.
    pub locked_reason: Option<String>,
    #[serde(rename = "spent_date")]
    ///Date the expense occurred.
    pub spent_date: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the expense was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the expense was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Expense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseCategory {
    #[serde(rename = "id")]
    ///Unique ID for the expense category.
    pub id: Option<i64>,
    #[serde(rename = "name")]
    ///The name of the expense category.
    pub name: Option<String>,
    #[serde(rename = "unit_name")]
    ///The unit name of the expense category.
    pub unit_name: Option<String>,
    #[serde(rename = "unit_price")]
    ///The unit price of the expense category.
    pub unit_price: Option<f64>,
    #[serde(rename = "is_active")]
    ///Whether the expense category is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "created_at")]
    ///Date and time the expense category was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the expense category was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for ExpenseCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "id")]
    ///Unique ID for the task.
    pub id: Option<i64>,
    #[serde(rename = "name")]
    ///The name of the task.
    pub name: Option<String>,
    #[serde(rename = "billable_by_default")]
    ///Used in determining whether default tasks should be marked billable when creating a new project.
    pub billable_by_default: Option<bool>,
    #[serde(rename = "default_hourly_rate")]
    ///The hourly rate to use for this task when it is added to a project.
    pub default_hourly_rate: Option<f64>,
    #[serde(rename = "is_default")]
    ///Whether this task should be automatically added to future projects.
    pub is_default: Option<bool>,
    #[serde(rename = "is_active")]
    ///Whether this task is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "created_at")]
    ///Date and time the task was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the task was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntry {
    #[serde(rename = "id")]
    ///Unique ID for the time entry.
    pub id: Option<i64>,
    #[serde(rename = "spent_date")]
    ///Date of the time entry.
    pub spent_date: Option<String>,
    #[serde(rename = "user")]
    ///An object containing the id and name of the associated user.
    pub user: Option<serde_json::Value>,
    #[serde(rename = "user_assignment")]
    pub user_assignment: Option<UserAssignment>,
    #[serde(rename = "client")]
    ///An object containing the id and name of the associated client.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "project")]
    ///An object containing the id and name of the associated project.
    pub project: Option<serde_json::Value>,
    #[serde(rename = "task")]
    ///An object containing the id and name of the associated task.
    pub task: Option<serde_json::Value>,
    #[serde(rename = "task_assignment")]
    pub task_assignment: Option<TaskAssignment>,
    #[serde(rename = "external_reference")]
    ///An object containing the id, group_id, account_id, permalink, service, and service_icon_url of the associated external reference.
    pub external_reference: Option<serde_json::Value>,
    #[serde(rename = "invoice")]
    ///Once the time entry has been invoiced, this field will include the associated invoice’s id and number.
    pub invoice: Option<serde_json::Value>,
    #[serde(rename = "hours")]
    ///Number of (decimal time) hours tracked in this time entry.
    pub hours: Option<f64>,
    #[serde(rename = "hours_without_timer")]
    ///Number of (decimal time) hours already tracked in this time entry, before the timer was last started.
    pub hours_without_timer: Option<f64>,
    #[serde(rename = "rounded_hours")]
    ///Number of (decimal time) hours tracked in this time entry used in summary reports and invoices. This value is rounded according to the Time Rounding setting in your Preferences.
    pub rounded_hours: Option<f64>,
    #[serde(rename = "notes")]
    ///Notes attached to the time entry.
    pub notes: Option<String>,
    #[serde(rename = "is_locked")]
    ///Whether or not the time entry has been locked.
    pub is_locked: Option<bool>,
    #[serde(rename = "locked_reason")]
    ///Why the time entry has been locked.
    pub locked_reason: Option<String>,
    #[serde(rename = "is_closed")]
    ///Whether or not the time entry has been approved via Timesheet Approval.
    pub is_closed: Option<bool>,
    #[serde(rename = "is_billed")]
    ///Whether or not the time entry has been marked as invoiced.
    pub is_billed: Option<bool>,
    #[serde(rename = "timer_started_at")]
    ///Date and time the timer was started (if tracking by duration). Use the ISO 8601 Format.
    pub timer_started_at: Option<String>,
    #[serde(rename = "started_time")]
    ///Time the time entry was started (if tracking by start/end times).
    pub started_time: Option<String>,
    #[serde(rename = "ended_time")]
    ///Time the time entry was ended (if tracking by start/end times).
    pub ended_time: Option<String>,
    #[serde(rename = "is_running")]
    ///Whether or not the time entry is currently running.
    pub is_running: Option<bool>,
    #[serde(rename = "billable")]
    ///Whether or not the time entry is billable.
    pub billable: Option<bool>,
    #[serde(rename = "budgeted")]
    ///Whether or not the time entry counts towards the project budget.
    pub budgeted: Option<bool>,
    #[serde(rename = "billable_rate")]
    ///The billable rate for the time entry.
    pub billable_rate: Option<f64>,
    #[serde(rename = "cost_rate")]
    ///The cost rate for the time entry.
    pub cost_rate: Option<f64>,
    #[serde(rename = "created_at")]
    ///Date and time the time entry was created. Use the ISO 8601 Format.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the time entry was last updated. Use the ISO 8601 Format.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignment {
    #[serde(rename = "id")]
    ///Unique ID for the user assignment.
    pub id: Option<i64>,
    #[serde(rename = "project")]
    ///An object containing the id, name, and code of the associated project.
    pub project: Option<serde_json::Value>,
    #[serde(rename = "user")]
    ///An object containing the id and name of the associated user.
    pub user: Option<serde_json::Value>,
    #[serde(rename = "is_active")]
    ///Whether the user assignment is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "is_project_manager")]
    ///Determines if the user has Project Manager permissions for the project.
    pub is_project_manager: Option<bool>,
    #[serde(rename = "use_default_rates")]
    ///Determines which billable rate(s) will be used on the project for this user when bill_by is People. When true, the project will use the user’s default billable rates. When false, the project will use the custom rate defined on this user assignment.
    pub use_default_rates: Option<bool>,
    #[serde(rename = "hourly_rate")]
    ///Custom rate used when the project’s bill_by is People and use_default_rates is false.
    pub hourly_rate: Option<f64>,
    #[serde(rename = "budget")]
    ///Budget used when the project’s budget_by is person.
    pub budget: Option<f64>,
    #[serde(rename = "created_at")]
    ///Date and time the user assignment was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the user assignment was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for UserAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignment {
    #[serde(rename = "id")]
    ///Unique ID for the task assignment.
    pub id: Option<i64>,
    #[serde(rename = "project")]
    ///An object containing the id, name, and code of the associated project.
    pub project: Option<serde_json::Value>,
    #[serde(rename = "task")]
    ///An object containing the id and name of the associated task.
    pub task: Option<serde_json::Value>,
    #[serde(rename = "is_active")]
    ///Whether the task assignment is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "billable")]
    ///Whether the task assignment is billable or not.  For example: if set to true, all time tracked on this project for the associated task will be marked as billable.
    pub billable: Option<bool>,
    #[serde(rename = "hourly_rate")]
    ///Rate used when the project’s bill_by is Tasks.
    pub hourly_rate: Option<f64>,
    #[serde(rename = "budget")]
    ///Budget used when the project’s budget_by is task or task_fees.
    pub budget: Option<f64>,
    #[serde(rename = "created_at")]
    ///Date and time the task assignment was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the task assignment was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for TaskAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "id")]
    ///Unique ID for the project.
    pub id: Option<i64>,
    #[serde(rename = "client")]
    ///An object containing the project’s client id, name, and currency.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "name")]
    ///Unique name for the project.
    pub name: Option<String>,
    #[serde(rename = "code")]
    ///The code associated with the project.
    pub code: Option<String>,
    #[serde(rename = "is_active")]
    ///Whether the project is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "is_billable")]
    ///Whether the project is billable or not.
    pub is_billable: Option<bool>,
    #[serde(rename = "is_fixed_fee")]
    ///Whether the project is a fixed-fee project or not.
    pub is_fixed_fee: Option<bool>,
    #[serde(rename = "bill_by")]
    ///The method by which the project is invoiced.
    pub bill_by: Option<String>,
    #[serde(rename = "hourly_rate")]
    ///Rate for projects billed by Project Hourly Rate.
    pub hourly_rate: Option<f64>,
    #[serde(rename = "budget")]
    ///The budget in hours for the project when budgeting by time.
    pub budget: Option<f64>,
    #[serde(rename = "budget_by")]
    ///The method by which the project is budgeted.
    pub budget_by: Option<String>,
    #[serde(rename = "budget_is_monthly")]
    ///Option to have the budget reset every month.
    pub budget_is_monthly: Option<bool>,
    #[serde(rename = "notify_when_over_budget")]
    ///Whether Project Managers should be notified when the project goes over budget.
    pub notify_when_over_budget: Option<bool>,
    #[serde(rename = "over_budget_notification_percentage")]
    ///Percentage value used to trigger over budget email alerts.
    pub over_budget_notification_percentage: Option<f64>,
    #[serde(rename = "over_budget_notification_date")]
    ///Date of last over budget notification. If none have been sent, this will be null.
    pub over_budget_notification_date: Option<String>,
    #[serde(rename = "show_budget_to_all")]
    ///Option to show project budget to all employees. Does not apply to Total Project Fee projects.
    pub show_budget_to_all: Option<bool>,
    #[serde(rename = "cost_budget")]
    ///The monetary budget for the project when budgeting by money.
    pub cost_budget: Option<f64>,
    #[serde(rename = "cost_budget_include_expenses")]
    ///Option for budget of Total Project Fees projects to include tracked expenses.
    pub cost_budget_include_expenses: Option<bool>,
    #[serde(rename = "fee")]
    ///The amount you plan to invoice for the project. Only used by fixed-fee projects.
    pub fee: Option<f64>,
    #[serde(rename = "notes")]
    ///Project notes.
    pub notes: Option<String>,
    #[serde(rename = "starts_on")]
    ///Date the project was started.
    pub starts_on: Option<String>,
    #[serde(rename = "ends_on")]
    ///Date the project will end.
    pub ends_on: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the project was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the project was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    #[serde(rename = "id")]
    ///Unique ID for the role.
    pub id: Option<i64>,
    #[serde(rename = "name")]
    ///The name of the role.
    pub name: Option<String>,
    #[serde(rename = "user_ids")]
    ///The IDs of the users assigned to this role.
    pub user_ids: Option<Vec<i64>>,
    #[serde(rename = "created_at")]
    ///Date and time the role was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the role was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillableRate {
    #[serde(rename = "id")]
    ///Unique ID for the billable rate.
    pub id: Option<i64>,
    #[serde(rename = "amount")]
    ///The amount of the billable rate.
    pub amount: Option<f64>,
    #[serde(rename = "start_date")]
    ///The date the billable rate is effective.
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The date the billable rate is no longer effective. This date is calculated by Harvest.
    pub end_date: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the billable rate was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the billable rate was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for BillableRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CostRate {
    #[serde(rename = "id")]
    ///Unique ID for the cost rate.
    pub id: Option<i64>,
    #[serde(rename = "amount")]
    ///The amount of the cost rate.
    pub amount: Option<f64>,
    #[serde(rename = "start_date")]
    ///The date the cost rate is effective.
    pub start_date: Option<String>,
    #[serde(rename = "end_date")]
    ///The date the cost rate is no longer effective. This date is calculated by Harvest.
    pub end_date: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the cost rate was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the cost rate was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for CostRate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAssignment {
    #[serde(rename = "id")]
    ///Unique ID for the project assignment.
    pub id: Option<i64>,
    #[serde(rename = "is_active")]
    ///Whether the project assignment is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "is_project_manager")]
    ///Determines if the user has Project Manager permissions for the project.
    pub is_project_manager: Option<bool>,
    #[serde(rename = "use_default_rates")]
    ///Determines which billable rate(s) will be used on the project for this user when bill_by is People. When true, the project will use the user’s default billable rates. When false, the project will use the custom rate defined on this user assignment.
    pub use_default_rates: Option<bool>,
    #[serde(rename = "hourly_rate")]
    ///Custom rate used when the project’s bill_by is People and use_default_rates is false.
    pub hourly_rate: Option<f64>,
    #[serde(rename = "budget")]
    ///Budget used when the project’s budget_by is person.
    pub budget: Option<f64>,
    #[serde(rename = "created_at")]
    ///Date and time the project assignment was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the project assignment was last updated.
    pub updated_at: Option<String>,
    #[serde(rename = "project")]
    ///An object containing the assigned project id, name, and code.
    pub project: Option<serde_json::Value>,
    #[serde(rename = "client")]
    ///An object containing the project’s client id and name.
    pub client: Option<serde_json::Value>,
    #[serde(rename = "task_assignments")]
    ///Array of task assignment objects associated with the project.
    pub task_assignments: Option<Vec<TaskAssignment>>,
}
impl std::fmt::Display for ProjectAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    ///Unique ID for the user.
    pub id: Option<i64>,
    #[serde(rename = "first_name")]
    ///The first name of the user.
    pub first_name: Option<String>,
    #[serde(rename = "last_name")]
    ///The last name of the user.
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    ///The email address of the user.
    pub email: Option<String>,
    #[serde(rename = "telephone")]
    ///The user’s telephone number.
    pub telephone: Option<String>,
    #[serde(rename = "timezone")]
    ///The user’s timezone.
    pub timezone: Option<String>,
    #[serde(rename = "has_access_to_all_future_projects")]
    ///Whether the user should be automatically added to future projects.
    pub has_access_to_all_future_projects: Option<bool>,
    #[serde(rename = "is_contractor")]
    ///Whether the user is a contractor or an employee.
    pub is_contractor: Option<bool>,
    #[serde(rename = "is_active")]
    ///Whether the user is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "weekly_capacity")]
    ///The number of hours per week this person is available to work in seconds, in half hour increments. For example, if a person’s capacity is 35 hours, the API will return 126000 seconds.
    pub weekly_capacity: Option<i64>,
    #[serde(rename = "default_hourly_rate")]
    ///The billable rate to use for this user when they are added to a project.
    pub default_hourly_rate: Option<f64>,
    #[serde(rename = "cost_rate")]
    ///The cost rate to use for this user when calculating a project’s costs vs billable amount.
    pub cost_rate: Option<f64>,
    #[serde(rename = "roles")]
    ///The role names assigned to this person.
    pub roles: Option<Vec<String>>,
    #[serde(rename = "avatar_url")]
    ///The URL to the user’s avatar image.
    pub avatar_url: Option<String>,
    #[serde(rename = "created_at")]
    ///Date and time the user was created.
    pub created_at: Option<String>,
    #[serde(rename = "updated_at")]
    ///Date and time the user was last updated.
    pub updated_at: Option<String>,
}
impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseReportsResult {
    #[serde(rename = "client_id")]
    ///The ID of the client associated with the reported expenses. Only returned in the Client and Project reports.
    pub client_id: Option<i64>,
    #[serde(rename = "client_name")]
    ///The name of the client associated with the reported expenses. Only returned in the Client and Project reports.
    pub client_name: Option<String>,
    #[serde(rename = "project_id")]
    ///The ID of the project associated with the reported expenses. Only returned in the Client and Project reports.
    pub project_id: Option<i64>,
    #[serde(rename = "project_name")]
    ///The name of the project associated with the reported expenses. Only returned in the Client and Project reports.
    pub project_name: Option<String>,
    #[serde(rename = "expense_category_id")]
    ///The ID of the expense category associated with the reported expenses. Only returned in the Expense Category report.
    pub expense_category_id: Option<i64>,
    #[serde(rename = "expense_category_name")]
    ///The name of the expense category associated with the reported expenses. Only returned in the Expense Category report.
    pub expense_category_name: Option<String>,
    #[serde(rename = "user_id")]
    ///The ID of the user associated with the reported expenses. Only returned in the Team report.
    pub user_id: Option<i64>,
    #[serde(rename = "user_name")]
    ///The name of the user associated with the reported expenses. Only returned in the Team report.
    pub user_name: Option<String>,
    #[serde(rename = "is_contractor")]
    ///The contractor status of the user associated with the reported expenses. Only returned in the Team report.
    pub is_contractor: Option<bool>,
    #[serde(rename = "total_amount")]
    ///The totaled cost for all expenses for the given timeframe, subject (client, project, expense category, or user), and currency.
    pub total_amount: Option<f64>,
    #[serde(rename = "billable_amount")]
    ///The totaled cost for billable expenses for the given timeframe, subject (client, project, expense category, or user), and currency.
    pub billable_amount: Option<f64>,
    #[serde(rename = "currency")]
    ///The currency code associated with the expenses for this result.
    pub currency: Option<String>,
}
impl std::fmt::Display for ExpenseReportsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UninvoicedReportResult {
    #[serde(rename = "client_id")]
    ///The ID of the client associated with the reported hours and expenses.
    pub client_id: Option<i64>,
    #[serde(rename = "client_name")]
    ///The name of the client associated with the reported hours and expenses.
    pub client_name: Option<String>,
    #[serde(rename = "project_id")]
    ///The ID of the project associated with the reported hours and expenses.
    pub project_id: Option<i64>,
    #[serde(rename = "project_name")]
    ///The name of the project associated with the reported hours and expenses.
    pub project_name: Option<String>,
    #[serde(rename = "currency")]
    ///The currency code associated with the tracked hours for this result.
    pub currency: Option<String>,
    #[serde(rename = "total_hours")]
    ///The total hours for the given timeframe and project. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub total_hours: Option<f64>,
    #[serde(rename = "uninvoiced_hours")]
    ///The total hours for the given timeframe and project that have not been invoiced. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub uninvoiced_hours: Option<f64>,
    #[serde(rename = "uninvoiced_expenses")]
    ///The total amount for billable expenses for the timeframe and project that have not been invoiced.
    pub uninvoiced_expenses: Option<f64>,
    #[serde(rename = "uninvoiced_amount")]
    ///The total amount (time and expenses) for the timeframe and project that have not been invoiced.
    pub uninvoiced_amount: Option<f64>,
}
impl std::fmt::Display for UninvoicedReportResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeReportsResult {
    #[serde(rename = "client_id")]
    ///The ID of the client associated with the reported hours. Only returned in the Client and Project reports.
    pub client_id: Option<i64>,
    #[serde(rename = "client_name")]
    ///The name of the client associated with the reported hours. Only returned in the Client and Project reports.
    pub client_name: Option<String>,
    #[serde(rename = "project_id")]
    ///The ID of the project associated with the reported hours. Only returned in the Client and Project reports.
    pub project_id: Option<i64>,
    #[serde(rename = "project_name")]
    ///The name of the project associated with the reported hours. Only returned in the Client and Project reports.
    pub project_name: Option<String>,
    #[serde(rename = "task_id")]
    ///The ID of the task associated with the reported hours. Only returned in the Task report.
    pub task_id: Option<i64>,
    #[serde(rename = "task_name")]
    ///The name of the task associated with the reported hours. Only returned in the Task report.
    pub task_name: Option<String>,
    #[serde(rename = "user_id")]
    ///The ID of the user associated with the reported hours. Only returned in the Team report.
    pub user_id: Option<i64>,
    #[serde(rename = "user_name")]
    ///The name of the user associated with the reported hours. Only returned in the Team report.
    pub user_name: Option<String>,
    #[serde(rename = "is_contractor")]
    ///The contractor status of the user associated with the reported hours. Only returned in the Team report.
    pub is_contractor: Option<bool>,
    #[serde(rename = "total_hours")]
    ///The totaled hours for the given timeframe, subject (client, project, task, or user), and currency. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub total_hours: Option<f64>,
    #[serde(rename = "billable_hours")]
    ///The totaled billable hours for the given timeframe, subject (client, project, task, or user), and currency. If Time Rounding is turned on, the hours will be rounded according to your settings.
    pub billable_hours: Option<f64>,
    #[serde(rename = "currency")]
    ///The currency code associated with the tracked hours for this result. Only visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub currency: Option<String>,
    #[serde(rename = "billable_amount")]
    ///The totaled billable amount for the billable hours above. Only visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub billable_amount: Option<f64>,
}
impl std::fmt::Display for TimeReportsResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectBudgetReportResult {
    #[serde(rename = "client_id")]
    ///The ID of the client associated with this project.
    pub client_id: Option<i64>,
    #[serde(rename = "client_name")]
    ///The name of the client associated with this project.
    pub client_name: Option<String>,
    #[serde(rename = "project_id")]
    ///The ID of the project.
    pub project_id: Option<i64>,
    #[serde(rename = "project_name")]
    ///The name of the project.
    pub project_name: Option<String>,
    #[serde(rename = "budget_is_monthly")]
    ///Whether the budget is reset every month.
    pub budget_is_monthly: Option<bool>,
    #[serde(rename = "budget_by")]
    ///The method by which the project is budgeted. Options: project (Hours Per Project), project_cost (Total Project Fees), task (Hours Per Task), task_fees (Fees Per Task), person (Hours Per Person), none (No Budget).
    pub budget_by: Option<String>,
    #[serde(rename = "is_active")]
    ///Whether the project is active or archived.
    pub is_active: Option<bool>,
    #[serde(rename = "budget")]
    ///The budget in hours or money for the project when budgeting by time. If the project is budgeted by money, this value will only be visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub budget: Option<f64>,
    #[serde(rename = "budget_spent")]
    ///The total hours or money spent against the project’s budget. If Time Rounding is turned on, the hours will be rounded according to your settings. If the project is budgeted by money, this value will only be visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub budget_spent: Option<f64>,
    #[serde(rename = "budget_remaining")]
    ///The total hours or money remaining in the project’s budget. If Time Rounding is turned on, the hours will be rounded according to your settings. If the project is budgeted by money, this value will only be visible to Administrators and Project Managers with the View billable rates and amounts permission.
    pub budget_remaining: Option<f64>,
}
impl std::fmt::Display for ProjectBudgetReportResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Contacts {
    #[serde(rename = "contacts")]
    pub contacts: Vec<Contact>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Contacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Clients {
    #[serde(rename = "clients")]
    pub clients: Vec<Client>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Clients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Companies {
    #[serde(rename = "companies")]
    pub companies: Vec<Company>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Companies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMessages {
    #[serde(rename = "invoice_messages")]
    pub invoice_messages: Vec<InvoiceMessage>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for InvoiceMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceMessageRecipients {
    #[serde(rename = "invoice_message_recipients")]
    pub invoice_message_recipients: Vec<InvoiceMessageRecipient>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for InvoiceMessageRecipients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoicePayments {
    #[serde(rename = "invoice_payments")]
    pub invoice_payments: Vec<InvoicePayment>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for InvoicePayments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Invoices {
    #[serde(rename = "invoices")]
    pub invoices: Vec<Invoice>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Invoices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceLineItems {
    #[serde(rename = "invoice_line_items")]
    pub invoice_line_items: Vec<InvoiceLineItem>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for InvoiceLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItemCategories {
    #[serde(rename = "invoice_item_categories")]
    pub invoice_item_categories: Vec<InvoiceItemCategory>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for InvoiceItemCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMessages {
    #[serde(rename = "estimate_messages")]
    pub estimate_messages: Vec<EstimateMessage>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for EstimateMessages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateMessageRecipients {
    #[serde(rename = "estimate_message_recipients")]
    pub estimate_message_recipients: Vec<EstimateMessageRecipient>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for EstimateMessageRecipients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Estimates {
    #[serde(rename = "estimates")]
    pub estimates: Vec<Estimate>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Estimates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateLineItems {
    #[serde(rename = "estimate_line_items")]
    pub estimate_line_items: Vec<EstimateLineItem>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for EstimateLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimateItemCategories {
    #[serde(rename = "estimate_item_categories")]
    pub estimate_item_categories: Vec<EstimateItemCategory>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for EstimateItemCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Expenses {
    #[serde(rename = "expenses")]
    pub expenses: Vec<Expense>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Expenses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseCategories {
    #[serde(rename = "expense_categories")]
    pub expense_categories: Vec<ExpenseCategory>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for ExpenseCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tasks {
    #[serde(rename = "tasks")]
    pub tasks: Vec<Task>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Tasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeEntries {
    #[serde(rename = "time_entries")]
    pub time_entries: Vec<TimeEntry>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for TimeEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignments {
    #[serde(rename = "user_assignments")]
    pub user_assignments: Vec<UserAssignment>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for UserAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskAssignments {
    #[serde(rename = "task_assignments")]
    pub task_assignments: Vec<TaskAssignment>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for TaskAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Projects {
    #[serde(rename = "projects")]
    pub projects: Vec<Project>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Projects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Roles {
    #[serde(rename = "roles")]
    pub roles: Vec<Role>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BillableRates {
    #[serde(rename = "billable_rates")]
    pub billable_rates: Vec<BillableRate>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for BillableRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CostRates {
    #[serde(rename = "cost_rates")]
    pub cost_rates: Vec<CostRate>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for CostRates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAssignments {
    #[serde(rename = "project_assignments")]
    pub project_assignments: Vec<ProjectAssignment>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for ProjectAssignments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "users")]
    pub users: Vec<User>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for Users {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExpenseReportsResults {
    #[serde(rename = "results")]
    pub results: Vec<ExpenseReportsResult>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for ExpenseReportsResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UninvoicedReportResults {
    #[serde(rename = "results")]
    pub results: Vec<UninvoicedReportResult>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for UninvoicedReportResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeReportsResults {
    #[serde(rename = "results")]
    pub results: Vec<TimeReportsResult>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for TimeReportsResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectBudgetReportResults {
    #[serde(rename = "results")]
    pub results: Vec<ProjectBudgetReportResult>,
    #[serde(rename = "per_page")]
    pub per_page: i64,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
    #[serde(rename = "total_entries")]
    pub total_entries: i64,
    #[serde(rename = "next_page")]
    pub next_page: Option<i64>,
    #[serde(rename = "previous_page")]
    pub previous_page: Option<i64>,
    #[serde(rename = "page")]
    pub page: i64,
    #[serde(rename = "links")]
    pub links: PaginationLinks,
}
impl std::fmt::Display for ProjectBudgetReportResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "code")]
    pub code: Option<i64>,
    #[serde(rename = "message")]
    pub message: Option<String>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaginationLinks {
    #[serde(rename = "first")]
    ///First page
    pub first: String,
    #[serde(rename = "last")]
    ///Last page
    pub last: String,
    #[serde(rename = "previous")]
    ///Previous page
    pub previous: Option<String>,
    #[serde(rename = "next")]
    ///Next page
    pub next: Option<String>,
}
impl std::fmt::Display for PaginationLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
