#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .retrieve_invoice_message_subject_and_body_for_specific_invoice(invoice_id)
        .thank_you(true)
        .reminder(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
