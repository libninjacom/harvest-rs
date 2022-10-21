#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let role_id = "your role id";
    let response = client
        .update_role(role_id)
        .name("your name")
        .user_ids(vec![1])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
