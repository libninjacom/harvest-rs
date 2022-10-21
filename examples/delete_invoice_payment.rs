#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let invoice_id = "your invoice id";
    let payment_id = "your payment id";
    let response = client
        .delete_invoice_payment(invoice_id, payment_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
