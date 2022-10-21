#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let estimate_id = "your estimate id";
    let response = client
        .create_estimate_message(estimate_id)
        .event_type("your event type")
        .recipients(vec![::serde_json::json!({})])
        .subject("your subject")
        .body("your body")
        .send_me_a_copy(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
