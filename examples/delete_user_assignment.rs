#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let project_id = "your project id";
    let user_assignment_id = "your user assignment id";
    let response = client
        .delete_user_assignment(project_id, user_assignment_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
