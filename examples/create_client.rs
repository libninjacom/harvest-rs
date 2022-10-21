#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_client()
        .name("your name")
        .is_active(true)
        .address("your address")
        .currency("your currency")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
