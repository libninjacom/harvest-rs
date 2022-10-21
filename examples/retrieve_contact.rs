#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let contact_id = "your contact id";
    let response = client.retrieve_contact(contact_id).send().await.unwrap();
    println!("{:#?}", response);
}
