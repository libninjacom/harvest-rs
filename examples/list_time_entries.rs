#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .list_time_entries()
        .user_id(1)
        .client_id(1)
        .project_id(1)
        .task_id(1)
        .external_reference_id("your external reference id")
        .is_billed(true)
        .is_running(true)
        .updated_since("your updated since")
        .from("your from")
        .to("your to")
        .page(1)
        .per_page(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
