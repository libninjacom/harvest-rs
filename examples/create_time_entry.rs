#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_time_entry()
        .user_id(1)
        .project_id(1)
        .task_id(1)
        .spent_date("your spent date")
        .started_time("your started time")
        .ended_time("your ended time")
        .notes("your notes")
        .external_reference(::serde_json::json!({}))
        .hours(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
