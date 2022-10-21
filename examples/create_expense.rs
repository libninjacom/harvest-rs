#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_expense()
        .user_id(1)
        .project_id(1)
        .expense_category_id(1)
        .spent_date("your spent date")
        .units(1)
        .total_cost(1.0)
        .notes("your notes")
        .billable(true)
        .receipt("your receipt")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
