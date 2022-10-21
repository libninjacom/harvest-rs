#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .list_active_project_assignments_for_the_currently_authenticated_user()
        .page(1)
        .per_page(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
