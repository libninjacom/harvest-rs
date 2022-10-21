#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let user_id = "your user id";
    let billable_rate_id = "your billable rate id";
    let response = client
        .retrieve_billable_rate(user_id, billable_rate_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
