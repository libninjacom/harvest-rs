#![allow(unused_imports)]
use harvest_api::HarvestClient;
use harvest_api::model::*;
#[tokio::main]
async fn main() {
    let client = HarvestClient::from_env();
    let expense_id = "your expense id";
    let response = client
        .update_expense(expense_id)
        .project_id(1)
        .expense_category_id(1)
        .spent_date("your spent date")
        .units(1)
        .total_cost(1.0)
        .notes("your notes")
        .billable(true)
        .receipt("your receipt")
        .delete_receipt(true)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
