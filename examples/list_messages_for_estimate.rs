#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let estimate_id = "your estimate id";
    let response = client
        .list_messages_for_estimate(estimate_id)
        .updated_since("your updated since")
        .page(1)
        .per_page(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
