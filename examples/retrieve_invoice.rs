#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let invoice_id = "your invoice id";
    let response = client.retrieve_invoice(invoice_id).send().await.unwrap();
    println!("{:#?}", response);
}
