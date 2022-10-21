#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let time_entry_id = "your time entry id";
    let response = client
        .update_time_entry(time_entry_id)
        .project_id(1)
        .task_id(1)
        .spent_date("your spent date")
        .started_time("your started time")
        .ended_time("your ended time")
        .hours(1.0)
        .notes("your notes")
        .external_reference(::serde_json::json!({}))
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
