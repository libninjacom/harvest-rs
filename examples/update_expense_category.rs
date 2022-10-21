#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let expense_category_id = "your expense category id";
    let response = client
        .update_expense_category(expense_category_id)
        .name("your name")
        .unit_name("your unit name")
        .unit_price(1.0)
        .is_active(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
