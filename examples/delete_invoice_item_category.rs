#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let invoice_item_category_id = "your invoice item category id";
    let response = client
        .delete_invoice_item_category(invoice_item_category_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
