#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_project()
        .client_id(1)
        .name("your name")
        .code("your code")
        .is_active(true)
        .is_billable(true)
        .is_fixed_fee(true)
        .bill_by("your bill by")
        .hourly_rate(1.0)
        .budget(1.0)
        .budget_by("your budget by")
        .budget_is_monthly(true)
        .notify_when_over_budget(true)
        .over_budget_notification_percentage(1.0)
        .show_budget_to_all(true)
        .cost_budget(1.0)
        .cost_budget_include_expenses(true)
        .fee(1.0)
        .notes("your notes")
        .starts_on("your starts on")
        .ends_on("your ends on")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
