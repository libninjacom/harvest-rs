#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .update_company()
        .wants_timestamp_timers(true)
        .weekly_capacity(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
