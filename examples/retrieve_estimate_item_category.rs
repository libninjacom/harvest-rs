#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let estimate_item_category_id = "your estimate item category id";
    let response = client
        .retrieve_estimate_item_category(estimate_item_category_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
