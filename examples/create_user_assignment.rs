#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let project_id = "your project id";
    let response = client
        .create_user_assignment(project_id)
        .user_id(1)
        .is_active(true)
        .is_project_manager(true)
        .use_default_rates(true)
        .hourly_rate(1.0)
        .budget(1.0)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
