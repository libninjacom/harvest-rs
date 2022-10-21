#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let user_id = "your user id";
    let response = client
        .create_billable_rate(user_id)
        .amount(1.0)
        .start_date("your start date")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
