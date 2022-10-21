#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let time_entry_id = "your time entry id";
    let response = client
        .delete_time_entry_external_reference(time_entry_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
