use chrono::Datelike;
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use url::Url;

use crate::HourlyPrice;
use crate::PriceRegion;

/// The blocking version of [`Strompris`].
///
/// Example:
/// ```rust
/// use strompris::blocking::Strompris;
/// use strompris::PriceRegion;
/// use strompris::Date;
///
/// fn main() {
///     let date = Date::from_ymd_opt(2024, 1, 31).unwrap();
///     let client = Strompris::default();
///     let resp = client.get_price(date, PriceRegion::NO1).unwrap();
///     for r in resp.iter() {
///         dbg!(r);
///     }
/// }
/// ```
/// [`Strompris`]: crate::Strompris
pub struct Strompris {
    client: Client,
    base_url: Url,
}

impl Strompris {
    fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());
        let client = Client::builder()
            .default_headers(headers)
            .https_only(true)
            .user_agent("Strømpris API wrapper written in Rust, github.com/erlingv-chr")
            .build()
            .unwrap();
        let base_url = Url::parse("https://www.hvakosterstrommen.no/api/v1/prices/").unwrap();
        Strompris { client, base_url }
    }

    /// Get the price for the given date and price region.
    pub fn get_price(
        &self,
        date: impl Datelike,
        price_region: PriceRegion,
    ) -> Result<Vec<HourlyPrice>, reqwest::Error> {

        let price_region = match price_region {
            PriceRegion::NO1 => "NO1",
            PriceRegion::NO2 => "NO2",
            PriceRegion::NO3 => "NO3",
            PriceRegion::NO4 => "NO4",
            PriceRegion::NO5 => "NO5",
        };

        let year = date.year();
        let month = date.month();
        let day = date.day();
        let endpoint = format!("{}/{:02}-{:02}_{}.json", year, month, day, price_region);
        let url = self.base_url.join(endpoint.as_str()).unwrap();
        self
            .client
            .get(url.as_str())
            .send()
            .unwrap()
            .json::<Vec<HourlyPrice>>()
    }
}

impl Default for Strompris {
    fn default() -> Self {
        Strompris::new()
    }
}
