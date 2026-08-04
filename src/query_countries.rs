use reqwest::Client;
use serde_json::Value;

pub async fn query_countries() -> Result<(), reqwest::Error> {
    let client = Client::new();

    let response = client
        .get("https://api.restcountries.com/countries/v5?q=canada")
        .header("Authorization", "Bearer <key>")
        .send()
        .await?;

    let data: Value = response.json().await?;

    println!("{}", data["data"]["objects"]);

    Ok(())
}
