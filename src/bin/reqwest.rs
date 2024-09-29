use rocket::serde::json::serde_json;
use rocket::tokio;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let resp = client
        .get("https://fakestoreapi.com/products?limit=2")
        .header("Content-type", "application/json")
        .send()
        .await?
        .json::<Vec<HashMap<String, serde_json::Value>>>()
        .await?;

    println!("{resp:#?}");

    Ok(())
}
