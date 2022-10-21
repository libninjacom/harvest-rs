#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let expense_id = "your expense id";
    let response = client.delete_expense(expense_id).send().await.unwrap();
    println!("{:#?}", response);
}
