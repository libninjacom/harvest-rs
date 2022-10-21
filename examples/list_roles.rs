#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client.list_roles().page(1).per_page(1).send().await.unwrap();
    println!("{:#?}", response);
}
