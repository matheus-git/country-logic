use reqwest::Client;
use serde::Deserialize;

const API_URL: &str = "https://api.restcountries.com/countries/v5";
const RESPONSE_FIELDS: &str = "names.common,codes.alpha_3,population,region,borders";

#[derive(Deserialize)]
struct Response {
    data: Data,
}

#[derive(Deserialize)]
struct Data {
    objects: Vec<ApiCountry>,
    meta: Meta,
}

#[derive(Deserialize)]
struct Meta {
    #[serde(default)]
    more: bool,
}

#[derive(Deserialize)]
struct ApiCountry {
    names: Names,
    codes: Codes,
    population: u64,
    region: String,
    #[serde(default)]
    borders: Vec<String>,
}

#[derive(Deserialize)]
struct Names {
    common: String,
}

#[derive(Deserialize)]
struct Codes {
    alpha_3: String,
}

#[derive(Debug)]
pub struct Country {
    pub name: String,
    pub alpha3: String,
    pub population: u64,
    pub region: String,
    pub borders: Vec<String>,
}

impl From<ApiCountry> for Country {
    fn from(country: ApiCountry) -> Self {
        Self {
            name: country.names.common,
            alpha3: country.codes.alpha_3,
            population: country.population,
            region: country.region,
            borders: country.borders,
        }
    }
}

pub async fn fetch_countries() -> Result<Vec<Country>, Box<dyn std::error::Error>> {
    let api_key = std::env::var("RESTCOUNTRIES_API_KEY").map_err(
        |_| "RESTCOUNTRIES_API_KEY is not set; define it in the environment or a local .env file",
    )?;
    let client = Client::new();
    let mut countries = Vec::new();
    let mut offset = 0;

    loop {
        let response: Response = client
            .get(API_URL)
            .bearer_auth(&api_key)
            .query(&[
                ("limit", "100".to_owned()),
                ("offset", offset.to_string()),
                ("response_fields", RESPONSE_FIELDS.to_owned()),
            ])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let more = response.data.meta.more;
        let page_size = response.data.objects.len();
        countries.extend(response.data.objects.into_iter().map(Country::from));
        if !more || page_size == 0 {
            return Ok(countries);
        }
        offset += 100;
    }
}
