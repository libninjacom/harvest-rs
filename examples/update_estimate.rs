#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let estimate_id = "your estimate id";
    let response = client
        .update_estimate(estimate_id)
        .client_id(1)
        .number("your number")
        .purchase_order("your purchase order")
        .tax(1.0)
        .tax2(1.0)
        .discount(1.0)
        .subject("your subject")
        .notes("your notes")
        .currency("your currency")
        .issue_date("your issue date")
        .line_items(vec![::serde_json::json!({})])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
