#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .project_budget_report()
        .page(1)
        .per_page(1)
        .is_active(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
