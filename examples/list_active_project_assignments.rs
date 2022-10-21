#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let user_id = "your user id";
    let response = client
        .list_active_project_assignments(user_id)
        .updated_since("your updated since")
        .page(1)
        .per_page(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
