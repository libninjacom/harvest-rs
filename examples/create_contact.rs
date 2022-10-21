#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let response = client
        .create_contact()
        .client_id(1)
        .title("your title")
        .first_name("your first name")
        .last_name("your last name")
        .email("your email")
        .phone_office("your phone office")
        .phone_mobile("your phone mobile")
        .fax("your fax")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
