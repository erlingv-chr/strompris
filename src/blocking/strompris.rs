use chrono::{Datelike, NaiveDate};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use url::Url;

use crate::PriceRegion;
use crate::Result;
use crate::StromprisError;
use crate::{HourlyPrice, MIN_DATE};

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
    ///
    /// Note: The API does not know the future! Tomorrow's prices are usually ready by 13:00,
    /// local time.
    pub fn get_price(&self, date: impl Datelike, price_region: PriceRegion) -> Result<Vec<HourlyPrice>> {
        let price_region = match price_region {
            PriceRegion::NO1 => "NO1",
            PriceRegion::NO2 => "NO2",
            PriceRegion::NO3 => "NO3",
            PriceRegion::NO4 => "NO4",
            PriceRegion::NO5 => "NO5",
        };

        if !self.date_after_min_date(&date) {
            return Err(StromprisError {
                message: "Date is before the minimum acceptable date".to_string(),
            });
        }

        let year = date.year();
        let month = date.month();
        let day = date.day();
        let endpoint = format!("{}/{:02}-{:02}_{}.json", year, month, day, price_region);
        let url = self.base_url.join(endpoint.as_str()).unwrap();
        let response = self.client.get(url.as_str()).send().unwrap();
        if response.status().is_client_error() {
            return Err(StromprisError {
                message: "Prices not yet available".to_string(),
            });
        }
        response
            .json::<Vec<HourlyPrice>>()
            .map_err(|e| StromprisError { message: e.to_string() })
    }

    fn date_after_min_date(&self, given_date: &impl Datelike) -> bool {
        let year = given_date.year();
        let month = given_date.month();
        let day = given_date.day();
        let given_datetime = NaiveDate::from_ymd_opt(year, month, day).unwrap();
        given_datetime >= MIN_DATE.unwrap()
    }
}

impl Default for Strompris {
    fn default() -> Self {
        Strompris::new()
    }
}
