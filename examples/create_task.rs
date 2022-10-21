#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_task()
        .name("your name")
        .billable_by_default(true)
        .default_hourly_rate(1.0)
        .is_default(true)
        .is_active(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
