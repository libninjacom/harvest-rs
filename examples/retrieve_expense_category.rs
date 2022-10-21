#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let expense_category_id = "your expense category id";
    let response = client
        .retrieve_expense_category(expense_category_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
