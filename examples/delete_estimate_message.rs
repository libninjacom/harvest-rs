#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let estimate_id = "your estimate id";
    let message_id = "your message id";
    let response = client
        .delete_estimate_message(estimate_id, message_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
