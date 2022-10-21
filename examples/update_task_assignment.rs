#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let project_id = "your project id";
    let task_assignment_id = "your task assignment id";
    let response = client
        .update_task_assignment(project_id, task_assignment_id)
        .is_active(true)
        .billable(true)
        .hourly_rate(1.0)
        .budget(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
