#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let invoice_id = "your invoice id";
    let response = client
        .create_invoice_message(invoice_id)
        .event_type("your event type")
        .recipients(vec![::serde_json::json!({})])
        .subject("your subject")
        .body("your body")
        .include_link_to_client_invoice(true)
        .attach_pdf(true)
        .send_me_a_copy(true)
        .thank_you(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
