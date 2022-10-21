#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let user_id = "your user id";
    let cost_rate_id = "your cost rate id";
    let response = client
        .retrieve_cost_rate(user_id, cost_rate_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
