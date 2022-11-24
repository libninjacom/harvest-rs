#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let user_id = "your user id";
    let response = client
        .update_user_assigned_teammates(user_id)
        .teammate_ids(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
