use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    objects: Vec<Country>,
    meta: Meta,
}

#[derive(Debug, Deserialize)]
struct Meta {
    count: u32,
    limit: u32,
    offset: u32,
    more: bool,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    names: Names,
    population: u64,
    region: String,
}

#[derive(Debug, Deserialize)]
struct Names {
    common: String,
}

pub async fn query_countries() -> Result<Vec<Country>, reqwest::Error> {
    let client = Client::new();

    let limit = 100;
    let mut offset = 0;
    let mut countries = Vec::new();

    loop {
        let url = format!(
            "https://api.restcountries.com/countries/v5?limit={limit}&offset={offset}"
        );

        let response: Response = client
            .get(url)
            .header("Authorization", "Bearer rc_live_c210b00fcc674d4f93f20cd23dc28e38")
            .send()
            .await?
            .json()
            .await?;

        let more = response.data.meta.more;

        countries.extend(response.data.objects);

        if !more {
            break;
        }

        offset += limit;
    }

    Ok(countries)
}
