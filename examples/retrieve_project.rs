#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let project_id = "your project id";
    let response = client.retrieve_project(project_id).send().await.unwrap();
    println!("{:#?}", response);
}
