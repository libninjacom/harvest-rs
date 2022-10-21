#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let client_id = "your client id";
    let response = client.retrieve_client(client_id).send().await.unwrap();
    println!("{:#?}", response);
}
