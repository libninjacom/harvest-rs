#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .list_invoices()
        .client_id(1)
        .project_id(1)
        .updated_since("your updated since")
        .from("your from")
        .to("your to")
        .state("your state")
        .page(1)
        .per_page(1)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
