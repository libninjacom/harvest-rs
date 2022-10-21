#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let contact_id = "your contact id";
    let response = client
        .update_contact(contact_id)
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
