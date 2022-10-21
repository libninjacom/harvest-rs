#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .create_invoice_payment(invoice_id)
        .amount(1.0)
        .paid_at("your paid at")
        .paid_date("your paid date")
        .notes("your notes")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
