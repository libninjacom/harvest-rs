#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let user_id = "your user id";
    let response = client
        .update_user(user_id)
        .first_name("your first name")
        .last_name("your last name")
        .email("your email")
        .timezone("your timezone")
        .has_access_to_all_future_projects(true)
        .is_contractor(true)
        .is_active(true)
        .weekly_capacity(1)
        .default_hourly_rate(1.0)
        .cost_rate(1.0)
        .roles(&["your roles"])
        .access_roles(&["your access roles"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
