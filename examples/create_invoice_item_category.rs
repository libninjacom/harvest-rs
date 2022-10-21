#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_invoice_item_category()
        .name("your name")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
